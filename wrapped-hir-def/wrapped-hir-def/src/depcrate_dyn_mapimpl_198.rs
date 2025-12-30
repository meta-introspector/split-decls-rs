// Generated macro for impl_198 (impl)
macro_rules! Depcrate_dyn_mapimpl_198 {
() => {
// Module: crate::dyn_map
// Provides: {"impl_198"}
// Dependencies: {}
impl < P : Policy > Index < Key < P :: K , P :: V , P > > for DynMap { type Output = KeyMap < Key < P :: K , P :: V , P > > ; fn index (& self , _key : Key < P :: K , P :: V , P >) -> & Self :: Output { unsafe { std :: mem :: transmute :: < & DynMap , & KeyMap < Key < P :: K , P :: V , P > > > (self) } } }
};
}
