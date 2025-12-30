// Generated macro for impl_161 (impl)
macro_rules! Depcrate_symbolsimpl_161 {
() => {
// Module: crate::symbols
// Provides: {"impl_161"}
// Dependencies: {}
impl Symbols { fn lit_suggestion (& self , lit : Lit) -> Option < (Span , String) > { if let LitKind :: Str (name , _) = lit . node { let sugg = if let Some ((prefix , name)) = self . symbol_map . get (& name . as_u32 ()) { format ! ("{prefix}::{name}") } else { format ! ("sym::{}" , name . as_str () . replace (| ch : char | ! ch . is_alphanumeric () , "_")) } ; Some ((lit . span , sugg)) } else { None } } fn expr_suggestion (& self , expr : & Expr < '_ >) -> Option < (Span , String) > { if let ExprKind :: Lit (lit) = expr . kind { self . lit_suggestion (lit) } else { None } } fn pat_suggestions (& self , pat : & Pat < '_ > , suggestions : & mut Vec < (Span , String) >) { pat . walk_always (| pat | { if let PatKind :: Expr (pat_expr) = pat . kind && let PatExprKind :: Lit { lit , .. } = pat_expr . kind { suggestions . extend (self . lit_suggestion (lit)) ; } }) ; } }
};
}
