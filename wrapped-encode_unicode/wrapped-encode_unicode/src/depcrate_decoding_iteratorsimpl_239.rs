// Generated macro for impl_239 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_239 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_239"}
// Dependencies: {}
impl < B : Borrow < u8 > , I : Iterator < Item = B > , T : IntoIterator < IntoIter = I , Item = B > > From < T > for Utf8CharMerger < B , I > { fn from (t : T) -> Self { Utf8CharMerger { iter : t . into_iter () , after_err_leftover : 0 , after_err_stack : [0 ; 3] , } } }
};
}
