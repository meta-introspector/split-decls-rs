// Generated macro for expr_has_unnecessary_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blocksexpr_has_unnecessary_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"expr_has_unnecessary_safety_comment"}
// Dependencies: {}
fn expr_has_unnecessary_safety_comment < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , comment_pos : BytePos ,) -> Option < Span > { if cx . tcx . hir_parent_iter (expr . hir_id) . any (| (_ , ref node) | { matches ! (node , Node :: Block (Block { rules : BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) , .. }) ,) }) { return None ; } if for_each_expr (cx , expr , | expr | match expr . kind { hir :: ExprKind :: Block (Block { rules : BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) , .. } , _ ,) => ControlFlow :: Break (()) , hir :: ExprKind :: Block (Block { rules : BlockCheckMode :: DefaultBlock , stmts : [hir :: Stmt { kind : hir :: StmtKind :: Let (hir :: LetStmt { source : hir :: LocalSource :: AssignDesugar , .. }) , .. } ,] , .. } , _ ,) => ControlFlow :: Continue (Descend :: Yes) , hir :: ExprKind :: Block (..) => ControlFlow :: Continue (Descend :: No) , _ => ControlFlow :: Continue (Descend :: Yes) , }) . is_some () { return None ; } let source_map = cx . tcx . sess . source_map () ; let span = Span :: new (comment_pos , comment_pos , SyntaxContext :: root () , None) ; let help_span = source_map . span_extend_to_next_char (span , '\n' , true) ; Some (help_span) }
};
}
