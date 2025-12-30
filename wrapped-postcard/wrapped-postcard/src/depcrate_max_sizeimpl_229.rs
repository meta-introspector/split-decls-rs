// Generated macro for impl_229 (impl)
macro_rules! Depcrate_max_sizeimpl_229 {
() => {
// Module: crate::max_size
// Provides: {"impl_229"}
// Dependencies: {}
# [cfg (feature = "heapless-v0_9")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless-v0_9")))] impl < const N : usize > MaxSize for heapless_v0_9 :: String < N > { const POSTCARD_MAX_SIZE : usize = < [u8 ; N] > :: POSTCARD_MAX_SIZE + varint_size (N) ; }
};
}
