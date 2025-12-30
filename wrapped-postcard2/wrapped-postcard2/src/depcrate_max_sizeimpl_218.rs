// Generated macro for impl_218 (impl)
macro_rules! Depcrate_max_sizeimpl_218 {
() => {
// Module: crate::max_size
// Provides: {"impl_218"}
// Dependencies: {}
# [cfg (feature = "heapless")] # [cfg_attr (docsrs , doc (cfg (feature = "heapless")))] impl < T : MaxSize , const N : usize > MaxSize for heapless :: Vec < T , N > { const POSTCARD_MAX_SIZE : usize = < [T ; N] > :: POSTCARD_MAX_SIZE + varint_size (N) ; }
};
}
