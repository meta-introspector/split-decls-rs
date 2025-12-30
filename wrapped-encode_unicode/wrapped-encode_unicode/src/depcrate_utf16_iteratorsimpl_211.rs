// Generated macro for impl_211 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_211 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_211"}
// Dependencies: {}
impl < U : Borrow < Utf16Char > , I : Iterator < Item = U > > Utf16CharSplitter < U , I > { # [doc = " Extracts the source iterator."] # [doc = ""] # [doc = " Note that `iter.into_inner().to_units()` is not a no-op:  "] # [doc = " If the last returned unit from `next()` was a leading surrogate,"] # [doc = " the trailing surrogate is lost."] pub fn into_inner (self) -> I { self . inner } }
};
}
