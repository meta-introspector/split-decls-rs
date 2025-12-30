// Generated macro for is_some_wild (function)
macro_rules! Depcrate_matches_match_like_matchesis_some_wild {
() => {
// Module: crate::matches::match_like_matches
// Provides: {"is_some_wild"}
// Dependencies: {}
# [doc = " Checks whether a pattern is `Some(_)`"] fn is_some_wild (pat_kind : PatKind < '_ >) -> bool { match pat_kind { PatKind :: TupleStruct (QPath :: Resolved (_ , path) , [first , ..] , _) if is_wild (first) => { let name = path . segments [0] . ident ; name . name == rustc_span :: sym :: Some } , _ => false , } }
};
}
