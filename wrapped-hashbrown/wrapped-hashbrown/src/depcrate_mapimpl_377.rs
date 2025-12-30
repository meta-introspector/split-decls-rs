// Generated macro for impl_377 (impl)
macro_rules! Depcrate_mapimpl_377 {
() => {
// Module: crate::map
// Provides: {"impl_377"}
// Dependencies: {}
impl < K : Debug , V : Debug , A : Allocator > fmt :: Debug for IntoIter < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
