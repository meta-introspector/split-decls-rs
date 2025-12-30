// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let coin = Coin :: Penny ; let mut count = 0 ; if let Coin :: Quarter (state) = coin { println ! ("State quarter from {state:?}!") ; } else { count += 1 ; } }
};
}
