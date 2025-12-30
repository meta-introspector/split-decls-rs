// Generated macro for impl_3231 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3231 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3231"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Similarity < Point3 < T > > for UnitQuaternion < T > { type Scaling = Id ; # [inline] fn translation (& self) -> Id { Id :: new () } # [inline] fn rotation (& self) -> Self { * self } # [inline] fn scaling (& self) -> Id { Id :: new () } }
};
}
