// Generated macro for len (function)
macro_rules! Depcrate_hpack_headerlen {
() => {
// Module: crate::hpack::header
// Provides: {"len"}
// Dependencies: {}
pub fn len (name : & HeaderName , value : & HeaderValue) -> usize { let n : & str = name . as_ref () ; 32 + n . len () + value . len () }
};
}
