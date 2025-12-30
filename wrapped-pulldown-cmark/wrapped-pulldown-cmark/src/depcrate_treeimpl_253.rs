// Generated macro for impl_253 (impl)
macro_rules! Depcrate_treeimpl_253 {
() => {
// Module: crate::tree
// Provides: {"impl_253"}
// Dependencies: {}
impl < T > core :: ops :: IndexMut < TreeIndex > for Tree < T > { fn index_mut (& mut self , ix : TreeIndex) -> & mut Node < T > { self . nodes . index_mut (ix . get ()) } }
};
}
