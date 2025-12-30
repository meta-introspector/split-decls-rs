// Generated macro for impl_126 (impl)
macro_rules! Depcrateimpl_126 {
() => {
// Module: crate
// Provides: {"impl_126"}
// Dependencies: {}
impl < T > fmt :: Binary for Complex < T > where T : fmt :: Binary + Num + PartialOrd + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write_complex ! (f , "b" , "0b" , self . re , self . im , T) } }
};
}
