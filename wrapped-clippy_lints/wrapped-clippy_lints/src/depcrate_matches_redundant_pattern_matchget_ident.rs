// Generated macro for get_ident (function)
macro_rules! Depcrate_matches_redundant_pattern_matchget_ident {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"get_ident"}
// Dependencies: {}
fn get_ident (path : & QPath < '_ >) -> Option < rustc_span :: symbol :: Ident > { match path { QPath :: Resolved (_ , path) => { let name = path . segments [0] . ident ; Some (name) } , QPath :: TypeRelative (..) => None , } }
};
}
