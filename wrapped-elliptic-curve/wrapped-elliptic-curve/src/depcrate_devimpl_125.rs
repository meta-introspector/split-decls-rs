// Generated macro for impl_125 (impl)
macro_rules! Depcrate_devimpl_125 {
() => {
// Module: crate::dev
// Provides: {"impl_125"}
// Dependencies: {}
impl CurveGroup for ProjectivePoint { type AffineRepr = AffinePoint ; fn to_affine (& self) -> AffinePoint { match self { Self :: FixedBaseOutput (scalar) => AffinePoint :: FixedBaseOutput (* scalar) , Self :: Other (affine) => * affine , _ => unimplemented ! () , } } }
};
}
