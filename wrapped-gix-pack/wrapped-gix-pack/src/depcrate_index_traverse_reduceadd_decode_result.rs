// Generated macro for add_decode_result (function)
macro_rules! Depcrate_index_traverse_reduceadd_decode_result {
() => {
// Module: crate::index::traverse::reduce
// Provides: {"add_decode_result"}
// Dependencies: {}
fn add_decode_result (lhs : & mut data :: decode :: entry :: Outcome , rhs : data :: decode :: entry :: Outcome) { lhs . num_deltas += rhs . num_deltas ; lhs . decompressed_size += rhs . decompressed_size ; lhs . compressed_size += rhs . compressed_size ; lhs . object_size += rhs . object_size ; }
};
}
