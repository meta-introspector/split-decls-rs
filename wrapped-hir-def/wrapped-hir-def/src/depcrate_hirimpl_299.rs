// Generated macro for impl_299 (impl)
macro_rules! Depcrate_hirimpl_299 {
() => {
// Module: crate::hir
// Provides: {"impl_299"}
// Dependencies: {}
impl ExprOrPatId { pub fn as_expr (self) -> Option < ExprId > { match self { Self :: ExprId (v) => Some (v) , _ => None , } } pub fn is_expr (& self) -> bool { matches ! (self , Self :: ExprId (_)) } pub fn as_pat (self) -> Option < PatId > { match self { Self :: PatId (v) => Some (v) , _ => None , } } pub fn is_pat (& self) -> bool { matches ! (self , Self :: PatId (_)) } }
};
}
