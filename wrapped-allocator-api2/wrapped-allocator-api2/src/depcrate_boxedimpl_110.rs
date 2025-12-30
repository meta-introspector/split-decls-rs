// Generated macro for impl_110 (impl)
macro_rules! Depcrate_boxedimpl_110 {
() => {
// Module: crate::boxed
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < A : Allocator > Extend < Box < str , A > > for alloc_crate :: string :: String { fn extend < I : IntoIterator < Item = Box < str , A > > > (& mut self , iter : I) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } }
};
}
