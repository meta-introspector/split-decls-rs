// Generated macro for chunk_info (function)
macro_rules! Depcrate_commonchunk_info {
() => {
// Module: crate::common
// Provides: {"chunk_info"}
// Dependencies: {}
pub fn chunk_info (intrinsic_count : usize) -> (usize , usize) { let available_parallelism = std :: thread :: available_parallelism () . unwrap () . get () ; let chunk_size = intrinsic_count . div_ceil (Ord :: min (available_parallelism , intrinsic_count)) ; (chunk_size , intrinsic_count . div_ceil (chunk_size)) }
};
}
