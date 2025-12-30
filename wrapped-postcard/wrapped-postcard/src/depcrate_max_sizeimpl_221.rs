// Generated macro for impl_221 (impl)
macro_rules! Depcrate_max_sizeimpl_221 {
() => {
// Module: crate::max_size
// Provides: {"impl_221"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] impl < T : MaxSize > MaxSize for Box < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE ; }
};
}
