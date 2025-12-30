// Generated macro for to_str_tuple (function)
macro_rules! Depcrate_messageto_str_tuple {
() => {
// Module: crate::message
// Provides: {"to_str_tuple"}
// Dependencies: {}
fn to_str_tuple (trailers : & MessageTrailers , index : usize) -> (& str , & str) { unsafe { let (rkey , rvalue) = to_raw_tuple (& trailers , index) ; let key = CStr :: from_ptr (rkey) . to_str () . unwrap () ; let value = CStr :: from_ptr (rvalue) . to_str () . unwrap () ; (key , value) } }
};
}
