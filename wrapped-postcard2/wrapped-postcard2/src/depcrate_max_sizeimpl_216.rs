// Generated macro for impl_216 (impl)
macro_rules! Depcrate_max_sizeimpl_216 {
() => {
// Module: crate::max_size
// Provides: {"impl_216"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "alloc" , target_has_atomic = "ptr"))))] impl < T : MaxSize > MaxSize for Arc < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE ; }
};
}
