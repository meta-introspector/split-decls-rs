// Generated macro for impl_120 (impl)
macro_rules! Depcrateimpl_120 {
() => {
// Module: crate
// Provides: {"impl_120"}
// Dependencies: {}
impl < T > fmt :: Display for Complex < T > where T : fmt :: Display + Num + PartialOrd + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write_complex ! (f , "" , "" , self . re , self . im , T) } }
};
}
