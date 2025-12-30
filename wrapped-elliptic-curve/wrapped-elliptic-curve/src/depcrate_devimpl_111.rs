// Generated macro for impl_111 (impl)
macro_rules! Depcrate_devimpl_111 {
() => {
// Module: crate::dev
// Provides: {"impl_111"}
// Dependencies: {}
impl < const N : usize > BatchNormalize < [ProjectivePoint ; N] > for ProjectivePoint { type Output = [AffinePoint ; N] ; fn batch_normalize (points : & [ProjectivePoint ; N]) -> [AffinePoint ; N] { array :: from_fn (| index | points [index] . into ()) } }
};
}
