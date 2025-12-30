// Generated macro for impl_112 (impl)
macro_rules! Depcrate_devimpl_112 {
() => {
// Module: crate::dev
// Provides: {"impl_112"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl BatchNormalize < [ProjectivePoint] > for ProjectivePoint { type Output = Vec < AffinePoint > ; fn batch_normalize (points : & [ProjectivePoint]) -> Vec < AffinePoint > { points . iter () . copied () . map (AffinePoint :: from) . collect () } }
};
}
