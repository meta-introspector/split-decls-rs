// Generated macro for impl_23 (impl)
macro_rules! Depcrate_binaryimpl_23 {
() => {
// Module: crate::binary
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Display for BinaryDeserializerError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let description = match self . kind { ErrorKind :: InvalidData => "Invalid resource bundle data" , ErrorKind :: ResourceTypeMismatch => "Resource did not match expected data type" , ErrorKind :: UnsupportedFormat => "Unsupported resource bundle format" , ErrorKind :: Unknown => "Unknown error" , } ; write ! (f , "{description}: {}" , self . message) } }
};
}
