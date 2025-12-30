// Generated macro for StmtList (struct)
macro_rules! Depcrate_astStmtList {
() => {
// Module: crate::ast
// Provides: {"StmtList"}
// Dependencies: {}
# [doc = " A list of statements. This corresponds to the `stmt_list` non-terminal of the"] # [doc = " grammar."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct StmtList < A > { # [doc = " The list of statements."] pub stmts : Vec < Stmt < A > > , }
};
}
