// Generated macro for impl_15 (impl)
macro_rules! Depcrate_attrimpl_15 {
() => {
// Module: crate::attr
// Provides: {"impl_15"}
// Dependencies: {}
impl SvalAttribute for LabelAttr { type Result = LabelValue ; fn try_from_expr (& self , expr : & Expr) -> Option < Self :: Result > { match expr { Expr :: Lit (lit) => Some (self . from_lit (& lit . lit)) , Expr :: Path (path) => Some (LabelValue :: Ident (quote ! (# path))) , _ => None , } } fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Str (ref s) = lit { LabelValue :: Const (s . value ()) } else { panic ! ("unexpected value") } } }
};
}
