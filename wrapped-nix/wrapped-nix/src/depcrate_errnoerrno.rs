// Generated macro for errno (function)
macro_rules! Depcrate_errnoerrno {
() => {
// Module: crate::errno
// Provides: {"errno"}
// Dependencies: {}
# [doc = " Returns the platform-specific value of errno"] # [deprecated (since = "0.28.0" , note = "please use `Errno::last_raw()` instead")] pub fn errno () -> i32 { Errno :: last_raw () }
};
}
