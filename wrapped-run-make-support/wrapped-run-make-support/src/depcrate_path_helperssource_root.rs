// Generated macro for source_root (function)
macro_rules! Depcrate_path_helperssource_root {
() => {
// Module: crate::path_helpers
// Provides: {"source_root"}
// Dependencies: {}
# [doc = " Path to the root `rust-lang/rust` source checkout."] # [must_use] pub fn source_root () -> PathBuf { env_var ("SOURCE_ROOT") . into () }
};
}
