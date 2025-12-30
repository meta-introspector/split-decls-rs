// Generated macro for is_system_llvm (function)
macro_rules! Depcrate_core_config_configis_system_llvm {
() => {
// Module: crate::core::config::config
// Provides: {"is_system_llvm"}
// Dependencies: {}
# [doc = " Returns `true` if this is an external version of LLVM not managed by bootstrap."] # [doc = " In particular, we expect llvm sources to be available when this is false."] # [doc = ""] # [doc = " NOTE: this is not the same as `!is_rust_llvm` when `llvm_has_patches` is set."] pub fn is_system_llvm < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , target_config : & HashMap < TargetSelection , Target > , llvm_from_ci : bool , target : TargetSelection ,) -> bool { let dwn_ctx = dwn_ctx . as_ref () ; match target_config . get (& target) { Some (Target { llvm_config : Some (_) , .. }) => { let ci_llvm = llvm_from_ci && is_host_target (& dwn_ctx . host_target , & target) ; ! ci_llvm } Some (Target { llvm_config : None , .. }) => false , None => false , } }
};
}
