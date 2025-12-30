// Generated macro for pollfd (struct)
macro_rules! Depcratepollfd {
() => {
// Module: crate
// Provides: {"pollfd"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone , Default)] pub struct pollfd { # [doc = " file descriptor"] pub fd : i32 , # [doc = " events to look for"] pub events : i16 , # [doc = " events returned"] pub revents : i16 , }
};
}
