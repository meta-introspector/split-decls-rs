// Generated macro for impl_136 (impl)
macro_rules! Depcrate_reducedimpl_136 {
() => {
// Module: crate::reduced
// Provides: {"impl_136"}
// Dependencies: {}
impl < T : PartialEq + Clone , R : Reducer < T > + Clone > Neg for & ReducedInt < T , R > { type Output = ReducedInt < T , R > ; # [inline] fn neg (self) -> Self :: Output { let a = self . r . neg (self . a . clone ()) ; ReducedInt { a , r : self . r . clone () , } } }
};
}
