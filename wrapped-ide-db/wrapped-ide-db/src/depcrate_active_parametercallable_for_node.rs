// Generated macro for callable_for_node (function)
macro_rules! Depcrate_active_parametercallable_for_node {
() => {
// Module: crate::active_parameter
// Provides: {"callable_for_node"}
// Dependencies: {}
pub fn callable_for_node < 'db > (sema : & Semantics < 'db , RootDatabase > , calling_node : & ast :: CallableExpr , offset : TextSize ,) -> Option < (hir :: Callable < 'db > , Option < usize >) > { let callable = match calling_node { ast :: CallableExpr :: Call (call) => sema . resolve_expr_as_callable (& call . expr () ?) , ast :: CallableExpr :: MethodCall (call) => sema . resolve_method_call_as_callable (call) , } ? ; let active_param = calling_node . arg_list () . map (| arg_list | { arg_list . syntax () . children_with_tokens () . filter_map (NodeOrToken :: into_token) . filter (| t | t . kind () == T ! [,]) . take_while (| t | t . text_range () . start () <= offset) . count () }) ; Some ((callable , active_param)) }
};
}
