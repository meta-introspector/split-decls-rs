// Generated macro for panic (function)
macro_rules! Depcrate_rt_locationpanic {
() => {
// Module: crate::rt::location
// Provides: {"panic"}
// Dependencies: {}
pub (super) fn panic (msg : impl ToString) -> PanicBuilder { PanicBuilder { msg : msg . to_string () , locations : Vec :: new () , } }
};
}
