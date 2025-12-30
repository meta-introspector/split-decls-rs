// Generated macro for impl_140 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_140 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Utf8Chars < 'a > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { fmtr . debug_tuple ("Utf8CharIndices") . field (& self . as_str ()) . finish () } }
};
}
