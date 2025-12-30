// Generated macro for impl_123 (impl)
macro_rules! Depcrateimpl_123 {
() => {
// Module: crate
// Provides: {"impl_123"}
// Dependencies: {}
impl < T > fmt :: LowerHex for Complex < T > where T : fmt :: LowerHex + Num + PartialOrd + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write_complex ! (f , "x" , "0x" , self . re , self . im , T) } }
};
}
