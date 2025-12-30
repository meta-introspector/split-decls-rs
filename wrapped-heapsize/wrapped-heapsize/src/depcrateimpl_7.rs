// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < T > HeapSize for [T] where T : HeapSize , { # [doc = " Sum of heap memory owned by each element of a dynamically sized slice of"] # [doc = " `T`."] fn heap_size_of_children (& self) -> usize { self . iter () . map (HeapSize :: heap_size_of_children) . sum () } }
};
}
