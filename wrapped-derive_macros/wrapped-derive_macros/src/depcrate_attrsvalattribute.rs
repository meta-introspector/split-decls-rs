// Generated macro for SvalAttribute (trait)
macro_rules! Depcrate_attrSvalAttribute {
() => {
// Module: crate::attr
// Provides: {"SvalAttribute"}
// Dependencies: {}
pub (crate) trait SvalAttribute : RawAttribute { type Result : 'static ; fn try_from_expr (& self , expr : & Expr) -> Option < Self :: Result > { if let Expr :: Lit (lit) = expr { Some (self . from_lit (& lit . lit)) } else { None } } fn from_lit (& self , lit : & Lit) -> Self :: Result ; }
};
}
