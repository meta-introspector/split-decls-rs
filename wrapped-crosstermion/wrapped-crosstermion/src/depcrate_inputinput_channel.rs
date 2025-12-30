// Generated macro for input_channel (function)
macro_rules! Depcrate_inputinput_channel {
() => {
// Module: crate::input
// Provides: {"input_channel"}
// Dependencies: {}
# [doc = " Return a receiver of user input events to avoid blocking the main thread."] pub fn input_channel () -> std :: sync :: mpsc :: Receiver < Event > { use std :: convert :: TryInto ; let (key_send , key_receive) = std :: sync :: mpsc :: sync_channel (0) ; std :: thread :: spawn (move | | -> Result < () , std :: io :: Error > { loop { let event = match continue_on_interrupt (crossterm :: event :: read ()) { Action :: Continue => continue , Action :: Result (res) => res ? , } ; if let Ok (event) = event . try_into () { if key_send . send (event) . is_err () { break ; } } } Ok (()) }) ; key_receive }
};
}
