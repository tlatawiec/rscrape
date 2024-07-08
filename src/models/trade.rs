use crate::models::politician::Politician;

pub struct Trade {
  pub politician: Politician,	// politician executing the trade
  pub trade_issuer: String,         // trade issuer
  pub publish_date: String,         // date trade was published
  pub traded_date: String,          // date trade was conducted
  pub price: String,		// price per share
  pub size: String,                 // size of trade
  pub reporting_gap: String,	// gap between trade and publishing
  pub buy: String,                  // buy or sell (true - buy | false - sell)
}

impl Trade {
  // trade constructor
  pub fn new(politician: Politician, trade_issuer: String, publish_date: String, traded_date: String, reporting_gap: String, size: String, price: String, buy: String) -> Trade {

    Trade { 
      politician,
      trade_issuer,   
      publish_date, 
      traded_date, 
      reporting_gap,
      size, 
      price, 
      buy 
    }
  }
    
    // print function for a trade
  pub fn print(&self) {
    self.politician.print();
    println!("\tIssuer: {}\n\tPublished: {}\n\tTraded: {}", self.trade_issuer, self.publish_date, self.traded_date);
    println!("\tPrice: {}\n\tSize: {}\n\tReported After: {} days\n\tType: {}", self.price, self.size, self.reporting_gap, self.buy);
    println!("]\n");
  }
}
