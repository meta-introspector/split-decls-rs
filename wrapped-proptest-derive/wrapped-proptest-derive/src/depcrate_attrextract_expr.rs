// Generated macro for extract_expr (function)
macro_rules! Depcrate_attrextract_expr {
() => {
// Module: crate::attr
// Provides: {"extract_expr"}
// Dependencies: {}
# [doc = " Extract expression out of literal if possible."] fn extract_expr (lit : Lit) -> Option < Expr > { match lit { Lit :: Str (lit) => lit . parse () . ok () , lit @ Lit :: Int (_) => Some (lit_to_expr (lit)) , _ => None , } }
};
}
