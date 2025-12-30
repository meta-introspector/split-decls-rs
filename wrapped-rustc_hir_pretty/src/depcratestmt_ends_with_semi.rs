// Generated macro for stmt_ends_with_semi (function)
macro_rules! Depcratestmt_ends_with_semi {
() => {
// Module: crate
// Provides: {"stmt_ends_with_semi"}
// Dependencies: {}
# [doc = " This statement requires a semicolon after it."] # [doc = " note that in one case (stmt_semi), we've already"] # [doc = " seen the semicolon, and thus don't need another."] fn stmt_ends_with_semi (stmt : & hir :: StmtKind < '_ >) -> bool { match * stmt { hir :: StmtKind :: Let (_) => true , hir :: StmtKind :: Item (_) => false , hir :: StmtKind :: Expr (e) => expr_requires_semi_to_be_stmt (e) , hir :: StmtKind :: Semi (..) => false , } }
};
}
