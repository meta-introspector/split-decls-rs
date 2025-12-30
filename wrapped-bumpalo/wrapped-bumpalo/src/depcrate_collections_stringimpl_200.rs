// Generated macro for impl_200 (impl)
macro_rules! Depcrate_collections_stringimpl_200 {
() => {
// Module: crate::collections::string
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'bump > Extend < char > for String < 'bump > { fn extend < I : IntoIterator < Item = char > > (& mut self , iter : I) { let iterator = iter . into_iter () ; let (lower_bound , _) = iterator . size_hint () ; self . reserve (lower_bound) ; for ch in iterator { self . push (ch) } } }
};
}
