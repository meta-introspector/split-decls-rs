// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let coin = Coin :: Penny ; let mut count = 0 ; match coin { Coin :: Quarter (state) => println ! ("State quarter from {state:?}!") , _ => count += 1 , } }
};
}
