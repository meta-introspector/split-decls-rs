// Generated macro for NESTED_TUPLE_CHUNK_SIZE (const)
macro_rules! Depcrate_astNESTED_TUPLE_CHUNK_SIZE {
() => {
// Module: crate::ast
// Provides: {"NESTED_TUPLE_CHUNK_SIZE"}
// Dependencies: {}
# [doc = " The `MAX - 1` tuple length `Arbitrary` is implemented for. After this number,"] # [doc = " tuples are expanded as nested tuples of up to `MAX` elements. The value should"] # [doc = " be kept in sync with the largest impl in `proptest/src/arbitrary/tuples.rs`."] const NESTED_TUPLE_CHUNK_SIZE : usize = 9 ;
};
}
