// Generated macro for impl_258 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_258 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'a > Debug for Utf16CharDecoder < 'a > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmtr , "Utf16CharDecoder {{ units[{}..]: {:?} }}" , self . index , self . as_slice ()) } }
};
}
