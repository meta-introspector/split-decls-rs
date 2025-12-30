// Generated macro for impl_3248 (impl)
macro_rules! Depcrate_third_party_alga_alga_rotationimpl_3248 {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"impl_3248"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > Similarity < Point < T , D > > for Rotation < T , D > { type Scaling = Id ; # [inline] fn translation (& self) -> Id { Id :: new () } # [inline] fn rotation (& self) -> Self { * self } # [inline] fn scaling (& self) -> Id { Id :: new () } }
};
}
