// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_core_downloadimpl_1246 {
() => {
// Module: crate::core::download
// Provides: {"impl_1246"}
// Dependencies: {}
impl < 'a > From < & 'a Config > for DownloadContext < 'a > { fn from (value : & 'a Config) -> Self { DownloadContext { path_modification_cache : value . path_modification_cache . clone () , src : & value . src , host_target : value . host_target , submodules : & value . submodules , patch_binaries_for_nix : value . patch_binaries_for_nix , exec_ctx : & value . exec_ctx , stage0_metadata : & value . stage0_metadata , llvm_assertions : value . llvm_assertions , bootstrap_cache_path : & value . bootstrap_cache_path , is_running_on_ci : value . is_running_on_ci , } } }
};
}
