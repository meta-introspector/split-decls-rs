// Generated macro for GccCiMode (enum)
macro_rules! Depcrate_core_configGccCiMode {
() => {
// Module: crate::core::config
// Provides: {"GccCiMode"}
// Dependencies: {}
# [doc = " Determines how will GCC be provided."] # [derive (Default , Clone)] pub enum GccCiMode { # [doc = " Build GCC from the local `src/gcc` submodule."] BuildLocally , # [doc = " Try to download GCC from CI."] # [doc = " If it is not available on CI, it will be built locally instead."] # [default] DownloadFromCi , }
};
}
