// Generated macro for Options (struct)
macro_rules! Depcrate_repository_cloneOptions {
() => {
// Module: crate::repository::clone
// Provides: {"Options"}
// Dependencies: {}
pub struct Options { pub format : OutputFormat , pub bare : bool , pub handshake_info : bool , pub no_tags : bool , pub shallow : gix :: remote :: fetch :: Shallow , pub ref_name : Option < gix :: refs :: PartialName > , }
};
}
