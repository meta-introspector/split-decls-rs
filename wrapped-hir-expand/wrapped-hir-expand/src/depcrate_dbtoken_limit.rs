// Generated macro for TOKEN_LIMIT (const)
macro_rules! Depcrate_dbTOKEN_LIMIT {
() => {
// Module: crate::db
// Provides: {"TOKEN_LIMIT"}
// Dependencies: {}
# [doc = " Total limit on the number of tokens produced by any macro invocation."] # [doc = ""] # [doc = " If an invocation produces more tokens than this limit, it will not be stored in the database and"] # [doc = " an error will be emitted."] # [doc = ""] # [doc = " Actual max for `analysis-stats .` at some point: 30672."] const TOKEN_LIMIT : usize = 2_097_152 ;
};
}
