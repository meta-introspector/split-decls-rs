// Generated macro for impl_82 (impl)
macro_rules! Depcrate_astimpl_82 {
() => {
// Module: crate::ast
// Provides: {"impl_82"}
// Dependencies: {}
impl < A > NodeStmt < A > { pub (crate) fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> NodeStmt < B > { NodeStmt { node : self . node , attr : self . attr . map (| a | a . filter_map_attr (f)) , } } # [doc = " Get the name of the `NodeStmt`, i.e. the identifier of the"] # [doc = " `NodeID` contained in the `NodeStmt`."] pub fn name (& self) -> & str { & self . node . id } pub (crate) fn get_node_id (& self) -> & NodeID { & self . node } }
};
}
