// Generated macro for impl_62 (impl)
macro_rules! Depcrate_astimpl_62 {
() => {
// Module: crate::ast
// Provides: {"impl_62"}
// Dependencies: {}
# [cfg (feature = "display")] impl < A > Display for AList < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { for attr in & self . elems { write ! (f , "{}; " , attr) ? ; } Ok (()) } }
};
}
