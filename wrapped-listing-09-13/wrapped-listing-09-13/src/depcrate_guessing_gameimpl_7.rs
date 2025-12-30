// Generated macro for impl_7 (impl)
macro_rules! Depcrate_guessing_gameimpl_7 {
() => {
// Module: crate::guessing_game
// Provides: {"impl_7"}
// Dependencies: {}
impl Guess { pub fn new (value : i32) -> Guess { if value < 1 || value > 100 { panic ! ("Guess value must be between 1 and 100, got {value}.") ; } Guess { value } } pub fn value (& self) -> i32 { self . value } }
};
}
