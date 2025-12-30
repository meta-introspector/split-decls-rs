// Generated macro for StatementStatus (enum)
macro_rules! Depcrate_statementStatementStatus {
() => {
// Module: crate::statement
// Provides: {"StatementStatus"}
// Dependencies: {}
# [doc = " Prepared statement status counters."] # [doc = ""] # [doc = " See `https://www.sqlite.org/c3ref/c_stmtstatus_counter.html`"] # [doc = " for explanations of each."] # [doc = ""] # [doc = " Note that depending on your version of SQLite, all of these"] # [doc = " may not be available."] # [repr (i32)] # [derive (Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub enum StatementStatus { # [doc = " Equivalent to `SQLITE_STMTSTATUS_FULLSCAN_STEP`"] FullscanStep = 1 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_SORT`"] Sort = 2 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_AUTOINDEX`"] AutoIndex = 3 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_VM_STEP`"] VmStep = 4 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_REPREPARE` (3.20.0)"] RePrepare = 5 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_RUN` (3.20.0)"] Run = 6 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_FILTER_MISS`"] FilterMiss = 7 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_FILTER_HIT`"] FilterHit = 8 , # [doc = " Equivalent to `SQLITE_STMTSTATUS_MEMUSED` (3.20.0)"] MemUsed = 99 , }
};
}
