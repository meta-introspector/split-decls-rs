// Generated macro for impl_3242 (impl)
macro_rules! Depcrate_third_party_alga_alga_rotationimpl_3242 {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"impl_3242"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > AbstractMagma < Multiplicative > for Rotation < T , D > { # [inline] fn operate (& self , rhs : & Self) -> Self { self * rhs } }
};
}
