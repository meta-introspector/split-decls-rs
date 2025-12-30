// Generated macro for impl_3147 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3147 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3147"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Similarity < Point3 < T > > for UnitDualQuaternion < T > { type Scaling = Id ; # [inline] fn translation (& self) -> Translation3 < T > { self . translation () } # [inline] fn rotation (& self) -> UnitQuaternion < T > { self . rotation () } # [inline] fn scaling (& self) -> Id { Id :: new () } }
};
}
