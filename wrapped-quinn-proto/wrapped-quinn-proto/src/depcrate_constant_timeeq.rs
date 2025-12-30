// Generated macro for eq (function)
macro_rules! Depcrate_constant_timeeq {
() => {
// Module: crate::constant_time
// Provides: {"eq"}
// Dependencies: {}
# [doc = " Compares byte strings in constant time."] pub (crate) fn eq (a : & [u8] , b : & [u8]) -> bool { a . len () == b . len () && constant_time_ne (a , b) == 0 }
};
}
