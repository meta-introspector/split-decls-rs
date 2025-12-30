// Generated macro for impl_217 (impl)
macro_rules! Depcrate_max_sizeimpl_217 {
() => {
// Module: crate::max_size
// Provides: {"impl_217"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] impl < T : MaxSize > MaxSize for Rc < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE ; }
};
}
