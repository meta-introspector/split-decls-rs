// Generated macro for Options (struct)
macro_rules! Depcrate_repository_fetchOptions {
() => {
// Module: crate::repository::fetch
// Provides: {"Options"}
// Dependencies: {}
pub struct Options { pub format : OutputFormat , pub dry_run : bool , pub remote : Option < String > , # [doc = " If non-empty, override all ref-specs otherwise configured in the remote"] pub ref_specs : Vec < BString > , pub shallow : gix :: remote :: fetch :: Shallow , pub handshake_info : bool , pub negotiation_info : bool , pub open_negotiation_graph : Option < std :: path :: PathBuf > , }
};
}
