// Generated macro for Stmt (struct)
macro_rules! Depcrate_astStmt {
() => {
// Module: crate::ast
// Provides: {"Stmt"}
// Dependencies: {}
# [doc = " A statement. No `attrs` or `tokens` fields because each `StmtKind` variant"] # [doc = " contains an AST node with those fields. (Except for `StmtKind::Empty`,"] # [doc = " which never has attrs or tokens)"] # [derive (Clone , Encodable , Decodable , Debug)] pub struct Stmt { pub id : NodeId , pub kind : StmtKind , pub span : Span , }
};
}
