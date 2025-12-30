// Generated macro for impl_562 (impl)
macro_rules! Depcrateimpl_562 {
() => {
// Module: crate
// Provides: {"impl_562"}
// Dependencies: {}
# [cfg (not (feature = "master"))] impl TargetInfo { fn cpu_supports (& self , _feature : & str) -> bool { false } fn supports_target_dependent_type (& self , typ : CType) -> bool { match typ { CType :: UInt128t | CType :: Int128t => { if self . supports_128bit_integers . load (Ordering :: SeqCst) { return true ; } } _ => () , } false } }
};
}
