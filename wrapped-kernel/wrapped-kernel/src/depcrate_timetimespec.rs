// Generated macro for timespec (struct)
macro_rules! Depcrate_timetimespec {
() => {
// Module: crate::time
// Provides: {"timespec"}
// Dependencies: {}
# [doc = " Represent the number of seconds and nanoseconds since"] # [doc = " the Epoch (1970-01-01 00:00:00 +0000 (UTC))"] # [derive (Copy , Clone , Debug , Default)] # [repr (C)] pub struct timespec { # [doc = " seconds"] pub tv_sec : time_t , # [doc = " nanoseconds"] pub tv_nsec : i32 , }
};
}
