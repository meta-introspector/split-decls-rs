// Generated macro for timespec (struct)
macro_rules! Depcratetimespec {
() => {
// Module: crate
// Provides: {"timespec"}
// Dependencies: {}
# [doc = " `timespec` is used by `clock_gettime` to retrieve the"] # [doc = " current time"] # [derive (Default , Copy , Clone , Debug)] # [repr (C)] pub struct timespec { # [doc = " seconds"] pub tv_sec : time_t , # [doc = " nanoseconds"] pub tv_nsec : i32 , }
};
}
