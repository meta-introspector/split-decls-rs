// Generated macro for to_bytes_tuple (function)
macro_rules! Depcrate_messageto_bytes_tuple {
() => {
// Module: crate::message
// Provides: {"to_bytes_tuple"}
// Dependencies: {}
fn to_bytes_tuple (trailers : & MessageTrailers , index : usize) -> (& [u8] , & [u8]) { unsafe { let (rkey , rvalue) = to_raw_tuple (& trailers , index) ; let key = CStr :: from_ptr (rkey) . to_bytes () ; let value = CStr :: from_ptr (rvalue) . to_bytes () ; (key , value) } }
};
}
