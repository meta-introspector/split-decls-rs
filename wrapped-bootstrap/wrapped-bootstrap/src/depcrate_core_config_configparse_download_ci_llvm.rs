// Generated macro for parse_download_ci_llvm (function)
macro_rules! Depcrate_core_config_configparse_download_ci_llvm {
() => {
// Module: crate::core::config::config
// Provides: {"parse_download_ci_llvm"}
// Dependencies: {}
pub fn parse_download_ci_llvm < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , rust_info : & channel :: GitInfo , download_rustc_commit : & Option < String > , download_ci_llvm : Option < StringOrBool > , asserts : bool ,) -> bool { let dwn_ctx = dwn_ctx . as_ref () ; let default = if dwn_ctx . is_running_on_ci { StringOrBool :: String ("if-unchanged" . to_string ()) } else { StringOrBool :: Bool (true) } ; let download_ci_llvm = download_ci_llvm . unwrap_or (default) ; let if_unchanged = | | { if rust_info . is_from_tarball () { println ! ("ERROR: 'if-unchanged' is only compatible with Git managed sources.") ; crate :: exit ! (1) ; } # [cfg (not (test))] update_submodule (dwn_ctx , rust_info , "src/llvm-project") ; let has_changes = has_changes_from_upstream (dwn_ctx , LLVM_INVALIDATION_PATHS) ; if has_changes { false } else { llvm :: is_ci_llvm_available_for_target (& dwn_ctx . host_target , asserts) } } ; match download_ci_llvm { StringOrBool :: Bool (b) => { if ! b && download_rustc_commit . is_some () { panic ! ("`llvm.download-ci-llvm` cannot be set to `false` if `rust.download-rustc` is set to `true` or `if-unchanged`.") ; } if b && dwn_ctx . is_running_on_ci { panic ! ("`llvm.download-ci-llvm` cannot be set to `true` on CI. Use `if-unchanged` instead.") ; } b && llvm :: is_ci_llvm_available_for_target (& dwn_ctx . host_target , asserts) } StringOrBool :: String (s) if s == "if-unchanged" => if_unchanged () , StringOrBool :: String (other) => { panic ! ("unrecognized option for download-ci-llvm: {other:?}") } } }
};
}
