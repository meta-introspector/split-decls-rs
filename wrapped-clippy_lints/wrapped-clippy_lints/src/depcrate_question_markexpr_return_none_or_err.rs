// Generated macro for expr_return_none_or_err (function)
macro_rules! Depcrate_question_markexpr_return_none_or_err {
() => {
// Module: crate::question_mark
// Provides: {"expr_return_none_or_err"}
// Dependencies: {}
fn expr_return_none_or_err (smbl : Symbol , cx : & LateContext < '_ > , expr : & Expr < '_ > , cond_expr : & Expr < '_ > , err_sym : Option < Symbol > ,) -> bool { match peel_blocks_with_stmt (expr) . kind { ExprKind :: Ret (Some (ret_expr)) => expr_return_none_or_err (smbl , cx , ret_expr , cond_expr , err_sym) , ExprKind :: Path (ref qpath) => match smbl { sym :: Option => cx . qpath_res (qpath , expr . hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionNone) , sym :: Result => expr . res_local_id () . is_some () && expr . res_local_id () == cond_expr . res_local_id () , _ => false , } , ExprKind :: Call (call_expr , [arg]) => { if smbl == sym :: Result && let ExprKind :: Path (QPath :: Resolved (_ , path)) = & call_expr . kind && let Some (segment) = path . segments . first () && let Some (err_sym) = err_sym && let ExprKind :: Path (QPath :: Resolved (_ , arg_path)) = & arg . kind && let Some (PathSegment { ident , .. }) = arg_path . segments . first () { return segment . ident . name == sym :: Err && err_sym == ident . name ; } false } , _ => false , } }
};
}
