// Generated macro for DownloadContext (struct)
macro_rules! Depcrate_core_downloadDownloadContext {
() => {
// Module: crate::core::download
// Provides: {"DownloadContext"}
// Dependencies: {}
# [doc = " Only should be used for pre config initialization downloads."] pub (crate) struct DownloadContext < 'a > { pub path_modification_cache : Arc < Mutex < HashMap < Vec < & 'static str > , PathFreshness > > > , pub src : & 'a Path , pub submodules : & 'a Option < bool > , pub host_target : TargetSelection , pub patch_binaries_for_nix : Option < bool > , pub exec_ctx : & 'a ExecutionContext , pub stage0_metadata : & 'a build_helper :: stage0_parser :: Stage0 , pub llvm_assertions : bool , pub bootstrap_cache_path : & 'a Option < PathBuf > , pub is_running_on_ci : bool , }
};
}
