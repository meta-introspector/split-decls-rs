// Generated macro for align_buf (function)
macro_rules! Depcrate_marshalledalign_buf {
() => {
// Module: crate::marshalled
// Provides: {"align_buf"}
// Dependencies: {}
pub fn align_buf (v : & mut Vec < u8 > , align : usize) { let vlen = v . len () ; let x = align_up (vlen , align) ; v . extend_from_slice (& ZEROS [.. (x - vlen)]) }
};
}
