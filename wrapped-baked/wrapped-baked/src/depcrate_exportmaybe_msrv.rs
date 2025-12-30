// Generated macro for maybe_msrv (function)
macro_rules! Depcrate_exportmaybe_msrv {
() => {
// Module: crate::export
// Provides: {"maybe_msrv"}
// Dependencies: {}
fn maybe_msrv () -> TokenStream { std :: option_env ! ("CARGO_PKG_RUST_VERSION") . map (| msrv | { quote ! { # [clippy :: msrv = # msrv] } }) . unwrap_or_default () }
};
}
