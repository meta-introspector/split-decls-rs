// Generated macro for find_innermost_closure (function)
macro_rules! Depcrate_redundant_closure_callfind_innermost_closure {
() => {
// Module: crate::redundant_closure_call
// Provides: {"find_innermost_closure"}
// Dependencies: {}
# [doc = " Tries to find the innermost closure:"] # [doc = " ```rust,ignore"] # [doc = " (|| || || || 42)()()()()"] # [doc = "  ^^^^^^^^^^^^^^          given this nested closure expression"] # [doc = "           ^^^^^          we want to return this closure"] # [doc = " ```"] # [doc = " It also has a parameter for how many steps to go in at most, so as to"] # [doc = " not take more closures than there are calls."] fn find_innermost_closure < 'tcx > (cx : & LateContext < 'tcx > , mut expr : & 'tcx hir :: Expr < 'tcx > , mut steps : usize ,) -> Option < (& 'tcx hir :: Expr < 'tcx > , & 'tcx hir :: FnDecl < 'tcx > , ty :: Asyncness , & 'tcx [Param < 'tcx >] ,) > { let mut data = None ; while let ExprKind :: Closure (closure) = expr . kind && let body = cx . tcx . hir_body (closure . body) && { let mut visitor = ReturnVisitor ; ! visitor . visit_expr (body . value) . is_break () } && steps > 0 { expr = body . value ; data = Some ((body . value , closure . fn_decl , if is_async_closure (body) { ty :: Asyncness :: Yes } else { ty :: Asyncness :: No } , body . params ,)) ; steps -= 1 ; } data }
};
}
