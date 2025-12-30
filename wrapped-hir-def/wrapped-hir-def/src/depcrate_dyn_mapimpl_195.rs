// Generated macro for impl_195 (impl)
macro_rules! Depcrate_dyn_mapimpl_195 {
() => {
// Module: crate::dyn_map
// Provides: {"impl_195"}
// Dependencies: {}
impl < P : Policy > Index < Key < P :: K , P :: V , P > > for DynMap { type Output = KeyMap < Key < P :: K , P :: V , P > > ; fn index (& self , _key : Key < P :: K , P :: V , P >) -> & Self :: Output { unsafe { std :: mem :: transmute :: < & DynMap , & KeyMap < Key < P :: K , P :: V , P > > > (self) } } }
};
}
