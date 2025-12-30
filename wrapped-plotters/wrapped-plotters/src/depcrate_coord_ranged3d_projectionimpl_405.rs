// Generated macro for impl_405 (impl)
macro_rules! Depcrate_coord_ranged3d_projectionimpl_405 {
() => {
// Module: crate::coord::ranged3d::projection
// Provides: {"impl_405"}
// Dependencies: {}
impl Mul < ProjectionMatrix > for ProjectionMatrix { type Output = ProjectionMatrix ; fn mul (self , other : ProjectionMatrix) -> ProjectionMatrix { let mut ret = ProjectionMatrix :: zero () ; for r in 0 .. 4 { for c in 0 .. 4 { for k in 0 .. 4 { ret . 0 [r] [c] += other . 0 [r] [k] * self . 0 [k] [c] ; } } } ret . normalize () ; ret } }
};
}
