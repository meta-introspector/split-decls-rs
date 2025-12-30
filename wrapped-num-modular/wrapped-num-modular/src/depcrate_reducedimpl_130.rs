// Generated macro for impl_130 (impl)
macro_rules! Depcrate_reducedimpl_130 {
() => {
// Module: crate::reduced
// Provides: {"impl_130"}
// Dependencies: {}
impl < T : PartialEq , R : Reducer < T > > PartialEq for ReducedInt < T , R > { # [inline] fn eq (& self , other : & Self) -> bool { self . check_modulus_eq (other) ; self . a == other . a } }
};
}
