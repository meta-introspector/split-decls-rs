// Generated macro for check_tt_count (function)
macro_rules! Depcrate_dbcheck_tt_count {
() => {
// Module: crate::db
// Provides: {"check_tt_count"}
// Dependencies: {}
fn check_tt_count (tt : & tt :: TopSubtree) -> Result < () , ExpandResult < () > > { let tt = tt . top_subtree () ; let count = tt . count () ; if count <= TOKEN_LIMIT { Ok (()) } else { Err (ExpandResult { value : () , err : Some (ExpandError :: other (tt . delimiter . open , format ! ("macro invocation exceeds token limit: produced {count} tokens, limit is {TOKEN_LIMIT}" ,) ,)) , }) } }
};
}
