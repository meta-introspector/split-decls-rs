// Generated macro for that_in_background (function)
macro_rules! Depcratethat_in_background {
() => {
// Module: crate
// Provides: {"that_in_background"}
// Dependencies: {}
# [doc = " Open path with the default application in a new thread to assure it's non-blocking."] # [doc = ""] # [doc = " See documentation of [`that()`] for more details."] pub fn that_in_background (path : impl AsRef < OsStr >) -> thread :: JoinHandle < io :: Result < () > > { let path = path . as_ref () . to_os_string () ; thread :: spawn (| | that (path)) }
};
}
