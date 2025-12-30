// Generated macro for impl_81 (impl)
macro_rules! Depcrate_adaptors_mapimpl_81 {
() => {
// Module: crate::adaptors::map
// Provides: {"impl_81"}
// Dependencies: {}
impl < I , R > DoubleEndedIterator for MapSpecialCase < I , R > where I : DoubleEndedIterator , R : MapSpecialCaseFn < I :: Item > , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| i | self . f . call (i)) } }
};
}
