// Generated macro for events (function)
macro_rules! Depcrate_termionevents {
() => {
// Module: crate::termion
// Provides: {"events"}
// Dependencies: {}
fn events (tick_rate : Duration) -> mpsc :: Receiver < Event > { let (tx , rx) = mpsc :: channel () ; let keys_tx = tx . clone () ; thread :: spawn (move | | { let stdin = io :: stdin () ; for key in stdin . keys () . flatten () { if let Err (err) = keys_tx . send (Event :: Input (key)) { eprintln ! ("{err}") ; return ; } } }) ; thread :: spawn (move | | { loop { if let Err (err) = tx . send (Event :: Tick) { eprintln ! ("{err}") ; break ; } thread :: sleep (tick_rate) ; } }) ; rx }
};
}
