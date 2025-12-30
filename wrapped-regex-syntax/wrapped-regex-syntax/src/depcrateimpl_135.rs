// Generated macro for impl_135 (impl)
macro_rules! Depcrateimpl_135 {
() => {
// Module: crate
// Provides: {"impl_135"}
// Dependencies: {}
impl fmt :: Display for ByteRange { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}-{}" , quote_byte (self . start) , quote_byte (self . end)) } }
};
}
