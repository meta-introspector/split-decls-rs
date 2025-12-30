// Generated macro for is_method (function)
macro_rules! Depcrate_methods_filter_mapis_method {
() => {
// Module: crate::methods::filter_map
// Provides: {"is_method"}
// Dependencies: {}
fn is_method (cx : & LateContext < '_ > , expr : & Expr < '_ > , method_name : Symbol) -> bool { match & expr . kind { ExprKind :: Path (QPath :: TypeRelative (_ , mname)) => mname . ident . name == method_name , ExprKind :: Path (QPath :: Resolved (_ , segments)) => segments . segments . last () . unwrap () . ident . name == method_name , ExprKind :: MethodCall (segment , _ , _ , _) => segment . ident . name == method_name , ExprKind :: Closure (Closure { body , .. }) => { let body = cx . tcx . hir_body (* body) ; let closure_expr = peel_blocks (body . value) ; match closure_expr . kind { ExprKind :: MethodCall (PathSegment { ident , .. } , receiver , ..) => { if ident . name == method_name && let ExprKind :: Path (path) = & receiver . kind && let Res :: Local (ref local) = cx . qpath_res (path , receiver . hir_id) && ! body . params . is_empty () { let arg_id = body . params [0] . pat . hir_id ; return arg_id == * local ; } false } , _ => false , } } , _ => false , } }
};
}
