// Generated macro for impl_129 (impl)
macro_rules! Depcrate_signature_bytesimpl_129 {
() => {
// Module: crate::signature::bytes
// Provides: {"impl_129"}
// Dependencies: {}
impl fmt :: Display for SignatureCompressed { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , BASE64_STANDARD . encode (self . 0)) } }
};
}
