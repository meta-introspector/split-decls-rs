// Generated macro for UNION_CHUNK_SIZE (const)
macro_rules! Depcrate_astUNION_CHUNK_SIZE {
() => {
// Module: crate::ast
// Provides: {"UNION_CHUNK_SIZE"}
// Dependencies: {}
# [doc = " The `MAX - 1` number of strategies that `TupleUnion` supports."] # [doc = " Increase this if the behaviour is changed in `proptest`."] # [doc = " Keeping this lower than what `proptest` supports will also work"] # [doc = " but for optimality this should follow what `proptest` supports."] # [cfg (not (feature = "boxed_union"))] const UNION_CHUNK_SIZE : usize = 9 ;
};
}
