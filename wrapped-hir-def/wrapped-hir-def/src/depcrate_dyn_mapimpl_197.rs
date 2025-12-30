// Generated macro for impl_197 (impl)
macro_rules! Depcrate_dyn_mapimpl_197 {
() => {
// Module: crate::dyn_map
// Provides: {"impl_197"}
// Dependencies: {}
impl < P : Policy > KeyMap < Key < P :: K , P :: V , P > > { pub fn insert (& mut self , key : P :: K , value : P :: V) { P :: insert (& mut self . map , key , value) } pub fn get (& self , key : & P :: K) -> Option < & P :: V > { P :: get (& self . map , key) } pub fn is_empty (& self) -> bool { P :: is_empty (& self . map) } }
};
}
