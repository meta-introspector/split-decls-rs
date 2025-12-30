// Generated macro for impl_114 (impl)
macro_rules! Depcrate_iterimpl_114 {
() => {
// Module: crate::iter
// Provides: {"impl_114"}
// Dependencies: {}
impl < I : Iterator > Iterator for Delimited < I > { type Item = IteratorItem < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { let item = IteratorItem { value : self . iter . next () ? , is_first : self . is_first , is_last : self . iter . peek () . is_none () , } ; self . is_first = false ; Some (item) } }
};
}
