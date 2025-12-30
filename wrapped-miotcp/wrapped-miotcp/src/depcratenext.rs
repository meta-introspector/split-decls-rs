// Generated macro for next (function)
macro_rules! Depcratenext {
() => {
// Module: crate
// Provides: {"next"}
// Dependencies: {}
fn next (current : & mut Token) -> Token { let next = current . 0 ; current . 0 += 1 ; Token (next) }
};
}
