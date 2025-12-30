// Generated macro for Stmt (enum)
macro_rules! Depcrate_subgraph_freeStmt {
() => {
// Module: crate::subgraph_free
// Provides: {"Stmt"}
// Dependencies: {}
# [doc = " A statement of the graph. This corresponds to the `stmt` non-terminal of the"] # [doc = " grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Stmt < A > { # [doc = " A node statement."] NodeStmt (NodeStmt < A >) , # [doc = " An edge statement."] EdgeStmt (EdgeStmt < A >) , # [doc = " An attribute statement."] AttrStmt (AttrStmt < A >) , # [doc = " An alias statement."] IDEq (String , String) , }
};
}
