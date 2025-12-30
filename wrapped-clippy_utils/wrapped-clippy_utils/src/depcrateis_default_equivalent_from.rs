// Generated macro for is_default_equivalent_from (function)
macro_rules! Depcrateis_default_equivalent_from {
() => {
// Module: crate
// Provides: {"is_default_equivalent_from"}
// Dependencies: {}
fn is_default_equivalent_from (cx : & LateContext < '_ > , from_func : & Expr < '_ > , arg : & Expr < '_ >) -> bool { if let ExprKind :: Path (QPath :: TypeRelative (ty , seg)) = from_func . kind && seg . ident . name == sym :: from { match arg . kind { ExprKind :: Lit (hir :: Lit { node : LitKind :: Str (sym , _) , .. }) => return sym . is_empty () && ty . basic_res () . is_lang_item (cx , LangItem :: String) , ExprKind :: Array ([]) => return ty . basic_res () . is_diag_item (cx , sym :: Vec) , ExprKind :: Repeat (_ , len) => { if let ConstArgKind :: Anon (anon_const) = len . kind && let ExprKind :: Lit (const_lit) = cx . tcx . hir_body (anon_const . body) . value . kind && let LitKind :: Int (v , _) = const_lit . node { return v == 0 && ty . basic_res () . is_diag_item (cx , sym :: Vec) ; } } , _ => () , } } false }
};
}
