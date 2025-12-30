// Generated macro for impl_16 (impl)
macro_rules! Depcrate_blockimpl_16 {
() => {
// Module: crate::block
// Provides: {"impl_16"}
// Dependencies: {}
impl darling :: FromMeta for BlockContents { fn from_value (value : & syn :: Lit) -> darling :: Result < Self > { if let syn :: Lit :: Str (s) = value { let contents = BlockContents :: try_from (s) ? ; if contents . is_empty () { Err (darling :: Error :: unknown_value ("") . with_span (s)) } else { Ok (contents) } } else { Err (darling :: Error :: unexpected_lit_type (value)) } } fn from_expr (expr : & syn :: Expr) -> darling :: Result < Self > { if let syn :: Expr :: Lit (lit) = expr { Self :: from_value (& lit . lit) } else { Ok (Self :: from (expr . clone ())) } } }
};
}
