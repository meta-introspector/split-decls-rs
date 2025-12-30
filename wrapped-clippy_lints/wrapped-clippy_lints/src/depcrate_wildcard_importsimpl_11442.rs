// Generated macro for impl_11442 (impl)
macro_rules! Depcrate_wildcard_importsimpl_11442 {
() => {
// Module: crate::wildcard_imports
// Provides: {"impl_11442"}
// Dependencies: {}
impl WildcardImports { pub fn new (conf : & 'static Conf) -> Self { Self { warn_on_all : conf . warn_on_all_wildcard_imports , allowed_segments : conf . allowed_wildcard_imports . iter () . cloned () . collect () , } } }
};
}
