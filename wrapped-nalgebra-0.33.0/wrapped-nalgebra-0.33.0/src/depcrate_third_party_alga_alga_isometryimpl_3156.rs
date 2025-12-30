// Generated macro for impl_3156 (impl)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_3156 {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_3156"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > Identity < Multiplicative > for Isometry < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] fn identity () -> Self { Self :: identity () } }
};
}
