// Generated macro for marker_impl (macro)
macro_rules! Depcrate_third_party_alga_alga_rotationmarker_impl {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"marker_impl"}
// Dependencies: {}
macro_rules ! marker_impl (($ ($ Trait : ident) ,*) => { $ (impl < T : RealField + simba :: scalar :: RealField , const D : usize > $ Trait < Point < T , D >> for Rotation < T , D > { }) * }) ;
};
}
