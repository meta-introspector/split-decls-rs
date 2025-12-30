// Generated macro for cargo_insta_version (function)
macro_rules! Depcrate_utilscargo_insta_version {
() => {
// Module: crate::utils
// Provides: {"cargo_insta_version"}
// Dependencies: {}
# [doc = " `cargo-insta` version (i.e. the binary that's currently running)."] pub (crate) fn cargo_insta_version () -> String { env ! ("CARGO_PKG_VERSION") . to_string () }
};
}
