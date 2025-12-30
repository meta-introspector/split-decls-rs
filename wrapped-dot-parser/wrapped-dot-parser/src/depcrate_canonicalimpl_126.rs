// Generated macro for impl_126 (impl)
macro_rules! Depcrate_canonicalimpl_126 {
() => {
// Module: crate::canonical
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (feature = "display")] impl < A > Display for Edge < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "\"{}\" -> \"{}\" [{}]" , self . from , self . to , self . attr) } }
};
}
