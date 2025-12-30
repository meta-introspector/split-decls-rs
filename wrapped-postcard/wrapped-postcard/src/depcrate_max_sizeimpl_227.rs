// Generated macro for impl_227 (impl)
macro_rules! Depcrate_max_sizeimpl_227 {
() => {
// Module: crate::max_size
// Provides: {"impl_227"}
// Dependencies: {}
# [cfg (feature = "heapless-v0_8")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless-v0_8")))] impl < const N : usize > MaxSize for heapless_v0_8 :: String < N > { const POSTCARD_MAX_SIZE : usize = < [u8 ; N] > :: POSTCARD_MAX_SIZE + varint_size (N) ; }
};
}
