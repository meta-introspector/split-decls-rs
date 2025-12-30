// Generated macro for impl_210 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_210 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_210"}
// Dependencies: {}
impl < U : Borrow < Utf16Char > , I : IntoIterator < Item = U > > From < I > for Utf16CharSplitter < U , I :: IntoIter > { fn from (iterable : I) -> Self { Utf16CharSplitter { inner : iterable . into_iter () , prev_second : 0 } } }
};
}
