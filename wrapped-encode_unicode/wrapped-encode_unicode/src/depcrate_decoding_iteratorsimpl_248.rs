// Generated macro for impl_248 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_248 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_248"}
// Dependencies: {}
impl < 'a > Debug for Utf8CharDecoder < 'a > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmtr , "Utf8CharDecoder {{ bytes[{}..]: {:?} }}" , self . index , self . as_slice ()) } }
};
}
