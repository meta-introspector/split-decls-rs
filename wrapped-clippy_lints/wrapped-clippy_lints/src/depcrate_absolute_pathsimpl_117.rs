// Generated macro for impl_117 (impl)
macro_rules! Depcrate_absolute_pathsimpl_117 {
() => {
// Module: crate::absolute_paths
// Provides: {"impl_117"}
// Dependencies: {}
impl AbsolutePaths { pub fn new (conf : & 'static Conf) -> Self { Self { absolute_paths_max_segments : conf . absolute_paths_max_segments , absolute_paths_allowed_crates : conf . absolute_paths_allowed_crates . iter () . map (| x | Symbol :: intern (x)) . collect () , } } }
};
}
