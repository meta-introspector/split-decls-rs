// Generated macro for ci_llvm_root (function)
macro_rules! Depcrate_core_config_configci_llvm_root {
() => {
// Module: crate::core::config::config
// Provides: {"ci_llvm_root"}
// Dependencies: {}
pub (crate) fn ci_llvm_root < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , llvm_from_ci : bool , out : & Path ,) -> PathBuf { let dwn_ctx = dwn_ctx . as_ref () ; assert ! (llvm_from_ci) ; out . join (dwn_ctx . host_target) . join ("ci-llvm") }
};
}
