// Generated macro for div_decode_result (function)
macro_rules! Depcrate_index_traverse_reducediv_decode_result {
() => {
// Module: crate::index::traverse::reduce
// Provides: {"div_decode_result"}
// Dependencies: {}
fn div_decode_result (lhs : & mut data :: decode :: entry :: Outcome , div : usize) { if div != 0 { lhs . num_deltas = (lhs . num_deltas as f32 / div as f32) as u32 ; lhs . decompressed_size /= div as u64 ; lhs . compressed_size /= div ; lhs . object_size /= div as u64 ; } }
};
}
