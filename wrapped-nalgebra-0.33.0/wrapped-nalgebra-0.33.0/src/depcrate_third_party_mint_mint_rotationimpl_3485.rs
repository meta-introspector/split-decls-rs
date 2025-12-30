// Generated macro for impl_3485 (impl)
macro_rules! Depcrate_third_party_mint_mint_rotationimpl_3485 {
() => {
// Module: crate::third_party::mint::mint_rotation
// Provides: {"impl_3485"}
// Dependencies: {}
impl < T : RealField > From < mint :: EulerAngles < T , mint :: IntraXYZ > > for Rotation3 < T > { fn from (euler : mint :: EulerAngles < T , mint :: IntraXYZ >) -> Self { Self :: from_euler_angles (euler . a , euler . b , euler . c) } }
};
}
