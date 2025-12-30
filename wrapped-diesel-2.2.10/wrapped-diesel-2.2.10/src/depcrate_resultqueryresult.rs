// Generated macro for QueryResult (type)
macro_rules! Depcrate_resultQueryResult {
() => {
// Module: crate::result
// Provides: {"QueryResult"}
// Dependencies: {}
# [doc = " A specialized result type for queries."] # [doc = ""] # [doc = " This type is exported by `diesel::prelude`, and is generally used by any"] # [doc = " code which is interacting with Diesel. This type exists to avoid writing out"] # [doc = " `diesel::result::Error`, and is otherwise a direct mapping to `Result`."] pub type QueryResult < T > = Result < T , Error > ;
};
}
