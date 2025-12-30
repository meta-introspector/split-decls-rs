// Generated macro for impl_120 (impl)
macro_rules! Depcrate_canonicalimpl_120 {
() => {
// Module: crate::canonical
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (feature = "display")] impl < A > Display for NodeSet < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { for node in self . set . values () { writeln ! (f , "{}" , node) ? ; } Ok (()) } }
};
}
