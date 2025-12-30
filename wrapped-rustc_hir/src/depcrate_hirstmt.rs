// Generated macro for Stmt (struct)
macro_rules! Depcrate_hirStmt {
() => {
// Module: crate::hir
// Provides: {"Stmt"}
// Dependencies: {}
# [doc = " A statement."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Stmt < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub kind : StmtKind < 'hir > , pub span : Span , }
};
}
