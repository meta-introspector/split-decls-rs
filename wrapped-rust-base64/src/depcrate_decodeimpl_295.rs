// Generated macro for impl_295 (impl)
macro_rules! Depcrate_decodeimpl_295 {
() => {
// Module: crate::decode
// Provides: {"impl_295"}
// Dependencies: {}
impl fmt :: Display for DecodeSliceError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: DecodeError (e) => write ! (f , "DecodeError: {}" , e) , Self :: OutputSliceTooSmall => write ! (f , "Output slice too small") , } } }
};
}
