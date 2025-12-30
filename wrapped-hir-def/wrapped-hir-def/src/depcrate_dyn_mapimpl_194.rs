// Generated macro for impl_194 (impl)
macro_rules! Depcrate_dyn_mapimpl_194 {
() => {
// Module: crate::dyn_map
// Provides: {"impl_194"}
// Dependencies: {}
impl < P : Policy > KeyMap < Key < P :: K , P :: V , P > > { pub fn insert (& mut self , key : P :: K , value : P :: V) { P :: insert (& mut self . map , key , value) } pub fn get (& self , key : & P :: K) -> Option < & P :: V > { P :: get (& self . map , key) } pub fn is_empty (& self) -> bool { P :: is_empty (& self . map) } }
};
}
