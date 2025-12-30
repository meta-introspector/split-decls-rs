// Generated macro for execute (function)
macro_rules! Depcrateexecute {
() => {
// Module: crate
// Provides: {"execute"}
// Dependencies: {}
pub async fn execute (query : & str , vars : Variables) -> QueryResult { let root = new_schema () ; let ctx = Context :: new () ; juniper :: execute (query , None , & root , & vars , & ctx) . await . map_err (| e | format ! ("{e:?}")) }
};
}
