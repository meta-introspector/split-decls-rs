// Generated macro for timeval (struct)
macro_rules! Depcrate_timetimeval {
() => {
// Module: crate::time
// Provides: {"timeval"}
// Dependencies: {}
# [doc = " Represent the number of seconds and microseconds since"] # [doc = " the Epoch (1970-01-01 00:00:00 +0000 (UTC))"] # [derive (Copy , Clone , Debug)] # [repr (C)] pub struct timeval { # [doc = " seconds"] pub tv_sec : time_t , # [doc = " microseconds"] pub tv_usec : suseconds_t , }
};
}
