// Generated macro for back_of_house (module)
macro_rules! Depcrateback_of_house {
() => {
// Module: crate
// Provides: {"back_of_house"}
// Dependencies: {}
mod back_of_house { pub struct Breakfast { pub toast : String , seasonal_fruit : String , } impl Breakfast { pub fn summer (toast : & str) -> Breakfast { Breakfast { toast : String :: from (toast) , seasonal_fruit : String :: from ("peaches") , } } } }
};
}
