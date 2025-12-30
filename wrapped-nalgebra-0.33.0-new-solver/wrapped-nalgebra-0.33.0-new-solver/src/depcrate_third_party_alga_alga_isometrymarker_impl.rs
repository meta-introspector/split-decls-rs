// Generated macro for marker_impl (macro)
macro_rules! Depcrate_third_party_alga_alga_isometrymarker_impl {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"marker_impl"}
// Dependencies: {}
macro_rules ! marker_impl (($ ($ Trait : ident) ,*) => { $ (impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > $ Trait < Point < T , D >> for Isometry < T , R , D > where R : Rotation < Point < T , D >> + AbstractRotation < T , D > { }) * }) ;
};
}
