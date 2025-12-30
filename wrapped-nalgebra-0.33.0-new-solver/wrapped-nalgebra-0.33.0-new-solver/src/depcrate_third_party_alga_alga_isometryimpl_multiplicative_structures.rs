// Generated macro for impl_multiplicative_structures (macro)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_multiplicative_structures {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_multiplicative_structures"}
// Dependencies: {}
macro_rules ! impl_multiplicative_structures (($ ($ marker : ident <$ operator : ident >) ,* $ (,) *) => { $ (impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > $ marker <$ operator > for Isometry < T , R , D > where R : Rotation < Point < T , D >> + AbstractRotation < T , D > { }) * }) ;
};
}
