// Generated macro for impl_structures (macro)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_structures {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_structures"}
// Dependencies: {}
macro_rules ! impl_structures (($ DualQuaternion : ident ; $ ($ marker : ident <$ operator : ident >) ,* $ (,) *) => { $ (impl < T : RealField + simba :: scalar :: RealField > $ marker <$ operator > for $ DualQuaternion < T > { }) * }) ;
};
}
