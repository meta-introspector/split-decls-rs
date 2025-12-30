// Generated macro for impl_250 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_250 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_250"}
// Dependencies: {}
impl < B : Borrow < u16 > , I : Iterator < Item = B > , T : IntoIterator < IntoIter = I , Item = B > > From < T > for Utf16CharMerger < B , I > { fn from (t : T) -> Self { Utf16CharMerger { iter : t . into_iter () , prev : None } } }
};
}
