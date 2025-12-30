// Generated macro for impl_126 (impl)
macro_rules! Depcrate_gv_parser_astimpl_126 {
() => {
// Module: crate::gv::parser::ast
// Provides: {"impl_126"}
// Dependencies: {}
impl EdgeStmt { pub fn new (from : NodeId) -> Self { Self { from , to : Vec :: new () , list : AttributeList :: new () , } } pub fn insert (& mut self , n : NodeId , ak : ArrowKind) { self . to . push ((n , ak)) ; } }
};
}
