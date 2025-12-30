// Generated macro for align_buf (function)
macro_rules! Depcrate_typesalign_buf {
() => {
// Module: crate::types
// Provides: {"align_buf"}
// Dependencies: {}
pub fn align_buf < M : Marshal > (a : & [u8]) -> Result < & [u8] , & 'static str > { let p = a . as_ptr () as usize ; let n = align_up (p , M :: ALIGN) ; let z = n - p ; if z >= a . len () { Err ("Not enough message data (while aligning)") ? } Ok (& a [(n - p) ..]) }
};
}
