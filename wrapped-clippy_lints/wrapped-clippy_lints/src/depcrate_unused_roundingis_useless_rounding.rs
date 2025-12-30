// Generated macro for is_useless_rounding (function)
macro_rules! Depcrate_unused_roundingis_useless_rounding {
() => {
// Module: crate::unused_rounding
// Provides: {"is_useless_rounding"}
// Dependencies: {}
fn is_useless_rounding (cx : & EarlyContext < '_ > , expr : & Expr) -> Option < (Symbol , String) > { if let ExprKind :: MethodCall (box MethodCall { seg : name_ident , receiver , .. }) = & expr . kind && let method_name = name_ident . ident . name && matches ! (method_name , sym :: ceil | sym :: floor | sym :: round) && let ExprKind :: Lit (token_lit) = & receiver . kind && token_lit . is_semantic_float () && let Ok (f) = token_lit . symbol . as_str () . replace ('_' , "") . parse :: < f64 > () && f . fract () == 0.0 { Some ((method_name , snippet (cx , receiver . span , "..") . into ())) } else { None } }
};
}
