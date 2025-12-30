// Generated macro for impl_199 (impl)
macro_rules! Depcrate_dyn_mapimpl_199 {
() => {
// Module: crate::dyn_map
// Provides: {"impl_199"}
// Dependencies: {}
impl < P : Policy > IndexMut < Key < P :: K , P :: V , P > > for DynMap { fn index_mut (& mut self , _key : Key < P :: K , P :: V , P >) -> & mut Self :: Output { unsafe { std :: mem :: transmute :: < & mut DynMap , & mut KeyMap < Key < P :: K , P :: V , P > > > (self) } } }
};
}
