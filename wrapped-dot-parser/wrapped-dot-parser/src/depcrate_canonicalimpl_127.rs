// Generated macro for impl_127 (impl)
macro_rules! Depcrate_canonicalimpl_127 {
() => {
// Module: crate::canonical
// Provides: {"impl_127"}
// Dependencies: {}
impl < A > Edge < A > { fn map < F , B > (self , f : F) -> Edge < B > where F : Fn (A) -> Option < B > , { Edge { from : self . from , to : self . to , attr : self . attr . filter_map_attr (& f) , } } }
};
}
