// Generated macro for impl_226 (impl)
macro_rules! Depcrate_max_sizeimpl_226 {
() => {
// Module: crate::max_size
// Provides: {"impl_226"}
// Dependencies: {}
# [cfg (feature = "heapless-v0_8")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless-v0_8")))] impl < T : MaxSize , const N : usize > MaxSize for heapless_v0_8 :: Vec < T , N > { const POSTCARD_MAX_SIZE : usize = < [T ; N] > :: POSTCARD_MAX_SIZE + varint_size (N) ; }
};
}
