// Generated macro for execute_sync (function)
macro_rules! Depcrateexecute_sync {
() => {
// Module: crate
// Provides: {"execute_sync"}
// Dependencies: {}
pub fn execute_sync (query : & str , vars : Variables) -> QueryResult { let root = new_schema () ; let ctx = Context :: new () ; juniper :: execute_sync (query , None , & root , & vars , & ctx) . map_err (| e | format ! ("{e:?}")) }
};
}
