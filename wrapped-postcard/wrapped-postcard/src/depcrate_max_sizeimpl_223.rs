// Generated macro for impl_223 (impl)
macro_rules! Depcrate_max_sizeimpl_223 {
() => {
// Module: crate::max_size
// Provides: {"impl_223"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] impl < T : MaxSize > MaxSize for Rc < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE ; }
};
}
