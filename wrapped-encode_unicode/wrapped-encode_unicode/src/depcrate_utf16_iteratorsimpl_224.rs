// Generated macro for impl_224 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_224 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Utf16Chars < 'a > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { fmtr . debug_tuple ("Utf16Chars") . field (& self . as_str ()) . finish () } }
};
}
