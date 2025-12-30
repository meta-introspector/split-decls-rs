// Generated macro for check_expression (function)
macro_rules! Depcrate_methods_unnecessary_filter_mapcheck_expression {
() => {
// Module: crate::methods::unnecessary_filter_map
// Provides: {"check_expression"}
// Dependencies: {}
fn check_expression < 'tcx > (cx : & LateContext < 'tcx > , arg_id : hir :: HirId , expr : & 'tcx hir :: Expr < '_ >) -> (bool , bool) { match expr . kind { hir :: ExprKind :: Path (ref path) if cx . qpath_res (path , expr . hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionNone) => { (false , true) } , hir :: ExprKind :: Call (func , args) => { if func . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionSome) { if args [0] . res_local_id () == Some (arg_id) { return (false , false) ; } return (true , false) ; } (true , true) } , hir :: ExprKind :: MethodCall (segment , recv , [arg] , _) => { if segment . ident . name == sym :: then_some && cx . typeck_results () . expr_ty (recv) . is_bool () && arg . res_local_id () == Some (arg_id) { (false , true) } else { (true , true) } } , hir :: ExprKind :: Block (block , _) => block . expr . as_ref () . map_or ((false , false) , | expr | check_expression (cx , arg_id , expr)) , hir :: ExprKind :: Match (_ , arms , _) => { let mut found_mapping = false ; let mut found_filtering = false ; for arm in arms { let (m , f) = check_expression (cx , arg_id , arm . body) ; found_mapping |= m ; found_filtering |= f ; } (found_mapping , found_filtering) } , hir :: ExprKind :: If (_ , if_arm , Some (else_arm)) => { let if_check = check_expression (cx , arg_id , if_arm) ; let else_check = check_expression (cx , arg_id , else_arm) ; (if_check . 0 | else_check . 0 , if_check . 1 | else_check . 1) } , _ => (true , true) , } }
};
}
