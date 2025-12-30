// Generated macro for manual_chunk (function)
macro_rules! Depcrate_commonmanual_chunk {
() => {
// Module: crate::common
// Provides: {"manual_chunk"}
// Dependencies: {}
pub fn manual_chunk (intrinsic_count : usize , chunk_size : usize) -> (usize , usize) { (chunk_size , intrinsic_count . div_ceil (chunk_size)) }
};
}
