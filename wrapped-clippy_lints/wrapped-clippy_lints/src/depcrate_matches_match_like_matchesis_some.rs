// Generated macro for is_some (function)
macro_rules! Depcrate_matches_match_like_matchesis_some {
() => {
// Module: crate::matches::match_like_matches
// Provides: {"is_some"}
// Dependencies: {}
fn is_some (path_kind : PatKind < '_ >) -> bool { match path_kind { PatKind :: TupleStruct (QPath :: Resolved (_ , path) , [first , ..] , _) if is_wild (first) => { let name = path . segments [0] . ident ; name . name == rustc_span :: sym :: Some } , _ => false , } }
};
}
