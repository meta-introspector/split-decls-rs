// Generated macro for impl_3158 (impl)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_3158 {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_3158"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > AbstractMagma < Multiplicative > for Isometry < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] fn operate (& self , rhs : & Self) -> Self { self * rhs } }
};
}
