// Generated macro for impl_252 (impl)
macro_rules! Depcrate_treeimpl_252 {
() => {
// Module: crate::tree
// Provides: {"impl_252"}
// Dependencies: {}
impl < T > core :: ops :: Index < TreeIndex > for Tree < T > { type Output = Node < T > ; fn index (& self , ix : TreeIndex) -> & Self :: Output { self . nodes . index (ix . get ()) } }
};
}
