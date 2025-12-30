// Generated macro for impl_3219 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3219 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3219"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > AbstractModule for Quaternion < T > { type AbstractRing = T ; # [inline] fn multiply_by (& self , n : T) -> Self { self * n } }
};
}
