// Generated macro for with_in_background (function)
macro_rules! Depcratewith_in_background {
() => {
// Module: crate
// Provides: {"with_in_background"}
// Dependencies: {}
# [doc = " Open path with the given application in a new thread, which is useful if"] # [doc = " the program ends up to be blocking. Otherwise, prefer [`with()`] for"] # [doc = " straightforward error handling."] # [doc = ""] # [doc = " See documentation of [`with()`] for more details."] pub fn with_in_background < T : AsRef < OsStr > > (path : T , app : impl Into < String > ,) -> thread :: JoinHandle < io :: Result < () > > { let path = path . as_ref () . to_os_string () ; let app = app . into () ; thread :: spawn (| | with (path , app)) }
};
}
