// Generated macro for to_raw_tuple (function)
macro_rules! Depcrate_messageto_raw_tuple {
() => {
// Module: crate::message
// Provides: {"to_raw_tuple"}
// Dependencies: {}
fn to_raw_tuple (trailers : & MessageTrailers , index : usize) -> (* const c_char , * const c_char) { unsafe { let addr = trailers . raw . trailers . wrapping_add (index) ; ((* addr) . key , (* addr) . value) } }
};
}
