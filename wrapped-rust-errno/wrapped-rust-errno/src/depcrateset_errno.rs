// Generated macro for set_errno (function)
macro_rules! Depcrateset_errno {
() => {
// Module: crate
// Provides: {"set_errno"}
// Dependencies: {}
# [doc = " Sets the platform-specific value of `errno`."] pub fn set_errno (err : Errno) { sys :: set_errno (err) }
};
}
