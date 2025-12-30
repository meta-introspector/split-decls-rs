// Generated macro for key_input_channel (function)
macro_rules! Depcrate_inputkey_input_channel {
() => {
// Module: crate::input
// Provides: {"key_input_channel"}
// Dependencies: {}
# [doc = " Return a receiver of user key input events to avoid blocking the main thread."] pub fn key_input_channel () -> std :: sync :: mpsc :: Receiver < Key > { let (key_send , key_receive) = std :: sync :: mpsc :: sync_channel (0) ; std :: thread :: spawn (move | | -> Result < () , std :: io :: Error > { loop { let event = match continue_on_interrupt (crossterm :: event :: read ()) { Action :: Continue => continue , Action :: Result (res) => res ? , } ; match event { crossterm :: event :: Event :: Key (key) => { if key_send . send (key) . is_err () { break ; } } _ => continue , } ; } Ok (()) }) ; key_receive }
};
}
