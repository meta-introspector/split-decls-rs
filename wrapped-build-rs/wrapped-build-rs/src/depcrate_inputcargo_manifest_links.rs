// Generated macro for cargo_manifest_links (function)
macro_rules! Depcrate_inputcargo_manifest_links {
() => {
// Module: crate::input
// Provides: {"cargo_manifest_links"}
// Dependencies: {}
# [doc = " The manifest `links` value."] # [track_caller] pub fn cargo_manifest_links () -> Option < String > { ENV . get ("CARGO_MANIFEST_LINKS") . map (to_string) }
};
}
