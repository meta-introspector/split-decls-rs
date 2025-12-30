// Generated macro for impl_122 (impl)
macro_rules! Depcrateimpl_122 {
() => {
// Module: crate
// Provides: {"impl_122"}
// Dependencies: {}
impl < T > fmt :: UpperExp for Complex < T > where T : fmt :: UpperExp + Num + PartialOrd + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write_complex ! (f , "E" , "" , self . re , self . im , T) } }
};
}
