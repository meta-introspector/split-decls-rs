// Generated macro for impl_135 (impl)
macro_rules! Depcrate_reducedimpl_135 {
() => {
// Module: crate::reduced
// Provides: {"impl_135"}
// Dependencies: {}
impl < T : PartialEq , R : Reducer < T > > Neg for ReducedInt < T , R > { type Output = Self ; # [inline] fn neg (self) -> Self :: Output { let Self { a , r } = self ; let a = r . neg (a) ; Self { a , r } } }
};
}
