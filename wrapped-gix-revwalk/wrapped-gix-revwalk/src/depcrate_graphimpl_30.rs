// Generated macro for impl_30 (impl)
macro_rules! Depcrate_graphimpl_30 {
() => {
// Module: crate::graph
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , T > Index < & 'a gix_hash :: oid > for Graph < '_ , '_ , T > { type Output = T ; fn index (& self , index : & 'a oid) -> & Self :: Output { & self . map [index] } }
};
}
