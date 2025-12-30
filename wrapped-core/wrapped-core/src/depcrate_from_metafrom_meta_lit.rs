// Generated macro for from_meta_lit (macro)
macro_rules! Depcrate_from_metafrom_meta_lit {
() => {
// Module: crate::from_meta
// Provides: {"from_meta_lit"}
// Dependencies: {}
macro_rules ! from_meta_lit { ($ impl_ty : path , $ lit_variant : path) => { impl FromMeta for $ impl_ty { fn from_value (value : & Lit) -> Result < Self > { if let $ lit_variant (ref value) = * value { Ok (value . clone ()) } else { Err (Error :: unexpected_lit_type (value)) } } } impl FromMeta for Vec <$ impl_ty > { fn from_list (items : & [NestedMeta]) -> Result < Self > { items . iter () . map (<$ impl_ty as FromMeta >:: from_nested_meta) . collect () } fn from_value (value : & syn :: Lit) -> Result < Self > { let expr_array = syn :: ExprArray :: from_value (value) ?; Self :: from_expr (& syn :: Expr :: Array (expr_array)) } fn from_expr (expr : & syn :: Expr) -> Result < Self > { match expr { syn :: Expr :: Array (expr_array) => expr_array . elems . iter () . map (<$ impl_ty as FromMeta >:: from_expr) . collect ::< Result < Vec < _ >>> () , syn :: Expr :: Lit (expr_lit) => Self :: from_value (& expr_lit . lit) , syn :: Expr :: Group (g) => Self :: from_expr (& g . expr) , _ => Err (Error :: unexpected_expr_type (expr)) , } } } } ; }
};
}
