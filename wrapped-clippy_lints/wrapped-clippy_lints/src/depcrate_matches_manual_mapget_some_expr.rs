// Generated macro for get_some_expr (function)
macro_rules! Depcrate_matches_manual_mapget_some_expr {
() => {
// Module: crate::matches::manual_map
// Provides: {"get_some_expr"}
// Dependencies: {}
fn get_some_expr < 'tcx > (cx : & LateContext < 'tcx > , _ : & 'tcx Pat < '_ > , expr : & 'tcx Expr < '_ > , ctxt : SyntaxContext ,) -> Option < SomeExpr < 'tcx > > { fn get_some_expr_internal < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , needs_unsafe_block : bool , ctxt : SyntaxContext ,) -> Option < SomeExpr < 'tcx > > { match expr . kind { ExprKind :: Call (callee , [arg]) if ctxt == expr . span . ctxt () && callee . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionSome) => { Some (SomeExpr :: new_no_negated (arg , needs_unsafe_block)) } , ExprKind :: Block (Block { stmts : [] , expr : Some (expr) , rules , .. } , _ ,) => get_some_expr_internal (cx , expr , needs_unsafe_block || * rules == BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) , ctxt ,) , _ => None , } } get_some_expr_internal (cx , expr , false , ctxt) }
};
}
