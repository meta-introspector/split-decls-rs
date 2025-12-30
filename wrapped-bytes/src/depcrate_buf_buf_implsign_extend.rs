// Generated macro for sign_extend (function)
macro_rules! Depcrate_buf_buf_implsign_extend {
() => {
// Module: crate::buf::buf_impl
// Provides: {"sign_extend"}
// Dependencies: {}
fn sign_extend (val : u64 , nbytes : usize) -> i64 { let shift = (8 - nbytes) * 8 ; (val << shift) as i64 >> shift }
};
}
