// Generated macro for impl_253 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_253 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_253"}
// Dependencies: {}
impl < B : Borrow < u16 > , I : Iterator < Item = B > + Debug > Debug for Utf16CharMerger < B , I > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { fmtr . debug_struct ("Utf16CharMerger") . field ("buffered" , & self . prev . as_ref () . map (| b | * b . borrow ())) . field ("inner" , & self . iter) . finish () } }
};
}
