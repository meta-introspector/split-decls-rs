// Generated macro for impl_9 (impl)
macro_rules! Depcrate_momoimpl_9 {
() => {
// Module: crate::momo
// Provides: {"impl_9"}
// Dependencies: {}
impl Conversion { fn conversion_expr (& self , i : & Ident) -> Expr { match * self { Conversion :: Into => parse_quote ! (# i . into ()) , Conversion :: AsRef => parse_quote ! (# i . as_ref ()) , Conversion :: AsMut => parse_quote ! (# i . as_mut ()) , } } }
};
}
