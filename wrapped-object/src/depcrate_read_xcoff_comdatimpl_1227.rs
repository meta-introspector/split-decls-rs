// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_read_xcoff_comdatimpl_1227 {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"impl_1227"}
// Dependencies: {}
impl < 'data , 'file , Xcoff , R > Iterator for XcoffComdatIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = XcoffComdat < 'data , 'file , Xcoff , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
};
}
