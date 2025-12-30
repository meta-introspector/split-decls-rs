// Generated macro for impl_124 (impl)
macro_rules! Depcrate_canonicalimpl_124 {
() => {
// Module: crate::canonical
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "display")] impl < A > Display for EdgeSet < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { for edge in & self . set { writeln ! (f , "{}" , edge) ? ; } Ok (()) } }
};
}
