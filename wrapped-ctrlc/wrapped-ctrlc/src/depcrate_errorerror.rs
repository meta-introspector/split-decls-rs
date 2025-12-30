// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Ctrl-C error."] # [derive (Debug)] pub enum Error { # [doc = " Signal could not be found from the system."] NoSuchSignal (crate :: SignalType) , # [doc = " Ctrl-C signal handler already registered."] MultipleHandlers , # [doc = " Unexpected system error."] System (std :: io :: Error) , }
};
}
