// Generated macro for impl_224 (impl)
macro_rules! Depcrate_max_sizeimpl_224 {
() => {
// Module: crate::max_size
// Provides: {"impl_224"}
// Dependencies: {}
# [cfg (feature = "heapless")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless")))] impl < T : MaxSize , const N : usize > MaxSize for heapless :: Vec < T , N > { const POSTCARD_MAX_SIZE : usize = < [T ; N] > :: POSTCARD_MAX_SIZE + varint_size (N) ; }
};
}
