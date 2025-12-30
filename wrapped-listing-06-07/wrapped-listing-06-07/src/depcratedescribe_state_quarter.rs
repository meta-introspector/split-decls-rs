// Generated macro for describe_state_quarter (function)
macro_rules! Depcratedescribe_state_quarter {
() => {
// Module: crate
// Provides: {"describe_state_quarter"}
// Dependencies: {}
fn describe_state_quarter (coin : Coin) -> Option < String > { if let Coin :: Quarter (state) = coin { if state . existed_in (1900) { Some (format ! ("{state:?} is pretty old, for America!")) } else { Some (format ! ("{state:?} is relatively new.")) } } else { None } }
};
}
