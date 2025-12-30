// Generated macro for rustc_version (function)
macro_rules! Depcrate_persist_file_formatrustc_version {
() => {
// Module: crate::persist::file_format
// Provides: {"rustc_version"}
// Dependencies: {}
# [doc = " A version string that hopefully is always different for compiler versions"] # [doc = " with different encodings of incremental compilation artifacts. Contains"] # [doc = " the Git commit hash."] fn rustc_version (nightly_build : bool , cfg_version : & 'static str) -> Cow < 'static , str > { if nightly_build { if let Ok (val) = env :: var ("RUSTC_FORCE_RUSTC_VERSION") { return val . into () ; } } cfg_version . into () }
};
}
