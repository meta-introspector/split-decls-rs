// Generated macro for impl_228 (impl)
macro_rules! Depcrate_max_sizeimpl_228 {
() => {
// Module: crate::max_size
// Provides: {"impl_228"}
// Dependencies: {}
# [cfg (feature = "heapless-v0_9")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless-v0_9")))] impl < T : MaxSize , const N : usize > MaxSize for heapless_v0_9 :: Vec < T , N > { const POSTCARD_MAX_SIZE : usize = < [T ; N] > :: POSTCARD_MAX_SIZE + varint_size (N) ; }
};
}
