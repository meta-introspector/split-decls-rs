// Generated macro for impl_146 (impl)
macro_rules! Depcrate_devimpl_146 {
() => {
// Module: crate::dev
// Provides: {"impl_146"}
// Dependencies: {}
impl Mul < Scalar > for ProjectivePoint { type Output = ProjectivePoint ; fn mul (self , scalar : Scalar) -> ProjectivePoint { match self { Self :: Generator => Self :: FixedBaseOutput (scalar) , _ => unimplemented ! () , } } }
};
}
