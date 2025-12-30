// Generated macro for impl_125 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_125 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_125"}
// Dependencies: {}
impl < U : Borrow < Utf8Char > , I : IntoIterator < Item = U > > From < I > for Utf8CharSplitter < U , I :: IntoIter > { fn from (iterable : I) -> Self { Utf8CharSplitter { inner : iterable . into_iter () , prev : 0 } } }
};
}
