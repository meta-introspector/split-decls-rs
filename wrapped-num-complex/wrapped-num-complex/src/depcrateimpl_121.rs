// Generated macro for impl_121 (impl)
macro_rules! Depcrateimpl_121 {
() => {
// Module: crate
// Provides: {"impl_121"}
// Dependencies: {}
impl < T > fmt :: LowerExp for Complex < T > where T : fmt :: LowerExp + Num + PartialOrd + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write_complex ! (f , "e" , "" , self . re , self . im , T) } }
};
}
