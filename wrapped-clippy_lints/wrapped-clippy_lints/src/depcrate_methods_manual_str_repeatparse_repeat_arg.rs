// Generated macro for parse_repeat_arg (function)
macro_rules! Depcrate_methods_manual_str_repeatparse_repeat_arg {
() => {
// Module: crate::methods::manual_str_repeat
// Provides: {"parse_repeat_arg"}
// Dependencies: {}
fn parse_repeat_arg (cx : & LateContext < '_ > , e : & Expr < '_ >) -> Option < RepeatKind > { if let ExprKind :: Lit (lit) = & e . kind { match lit . node { LitKind :: Str (..) => Some (RepeatKind :: String) , LitKind :: Char (c) => Some (RepeatKind :: Char (c)) , _ => None , } } else { let ty = cx . typeck_results () . expr_ty (e) ; if ty . is_lang_item (cx , LangItem :: String) || (ty . is_lang_item (cx , LangItem :: OwnedBox) && get_ty_param (ty) . is_some_and (Ty :: is_str)) || (ty . is_diag_item (cx , sym :: Cow) && get_ty_param (ty) . is_some_and (Ty :: is_str)) { Some (RepeatKind :: String) } else { let ty = ty . peel_refs () ; (ty . is_str () || ty . is_lang_item (cx , LangItem :: String)) . then_some (RepeatKind :: String) } } }
};
}
