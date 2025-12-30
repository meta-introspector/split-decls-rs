// Generated macro for marker_impl (macro)
macro_rules! Depcrate_third_party_alga_alga_quaternionmarker_impl {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"marker_impl"}
// Dependencies: {}
macro_rules ! marker_impl (($ ($ Trait : ident) ,*) => { $ (impl < T : RealField + simba :: scalar :: RealField > $ Trait < Point3 < T >> for UnitQuaternion < T > { }) * }) ;
};
}
