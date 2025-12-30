// Generated macro for impl_116 (impl)
macro_rules! Depcrate_devimpl_116 {
() => {
// Module: crate::dev
// Provides: {"impl_116"}
// Dependencies: {}
impl From < AffinePoint > for ProjectivePoint { fn from (point : AffinePoint) -> ProjectivePoint { match point { AffinePoint :: FixedBaseOutput (scalar) => ProjectivePoint :: FixedBaseOutput (scalar) , AffinePoint :: Identity => ProjectivePoint :: Identity , AffinePoint :: Generator => ProjectivePoint :: Generator , other => ProjectivePoint :: Other (other) , } } }
};
}
