// Generated macro for impl_9 (impl)
macro_rules! Depcrate_attrimpl_9 {
() => {
// Module: crate::attr
// Provides: {"impl_9"}
// Dependencies: {}
impl SvalAttribute for TagAttr { type Result = syn :: Path ; fn try_from_expr (& self , expr : & Expr) -> Option < Self :: Result > { match expr { Expr :: Lit (lit) => Some (self . from_lit (& lit . lit)) , Expr :: Path (path) => Some (path . path . clone ()) , _ => None , } } fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Str (ref s) = lit { s . parse () . expect ("invalid value") } else { panic ! ("unexpected value") } } }
};
}
