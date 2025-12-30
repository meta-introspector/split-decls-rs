// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl < T > fmt :: UpperHex for Complex < T > where T : fmt :: UpperHex + Num + PartialOrd + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write_complex ! (f , "X" , "0x" , self . re , self . im , T) } }
};
}
