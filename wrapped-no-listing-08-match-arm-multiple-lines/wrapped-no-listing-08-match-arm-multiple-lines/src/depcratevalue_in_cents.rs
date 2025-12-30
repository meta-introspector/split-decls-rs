// Generated macro for value_in_cents (function)
macro_rules! Depcratevalue_in_cents {
() => {
// Module: crate
// Provides: {"value_in_cents"}
// Dependencies: {}
fn value_in_cents (coin : Coin) -> u8 { match coin { Coin :: Penny => { println ! ("Lucky penny!") ; 1 } Coin :: Nickel => 5 , Coin :: Dime => 10 , Coin :: Quarter => 25 , } }
};
}
