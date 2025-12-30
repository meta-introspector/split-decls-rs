// Generated macro for impl_219 (impl)
macro_rules! Depcrate_max_sizeimpl_219 {
() => {
// Module: crate::max_size
// Provides: {"impl_219"}
// Dependencies: {}
# [cfg (feature = "heapless")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless")))] impl < const N : usize > MaxSize for heapless :: String < N > { const POSTCARD_MAX_SIZE : usize = < [u8 ; N] > :: POSTCARD_MAX_SIZE + varint_size (N) ; }
};
}
