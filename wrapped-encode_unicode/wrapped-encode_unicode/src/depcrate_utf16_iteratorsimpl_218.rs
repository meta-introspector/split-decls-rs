// Generated macro for impl_218 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_218 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Utf16CharIndices < 'a > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { fmtr . debug_tuple ("Utf16CharIndices") . field (& self . index) . field (& self . as_str ()) . finish () } }
};
}
