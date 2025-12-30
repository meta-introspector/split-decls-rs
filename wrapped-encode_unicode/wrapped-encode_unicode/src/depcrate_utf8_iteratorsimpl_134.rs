// Generated macro for impl_134 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_134 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Utf8CharIndices < 'a > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { fmtr . debug_tuple ("Utf8CharIndices") . field (& self . index) . field (& self . as_str ()) . finish () } }
};
}
