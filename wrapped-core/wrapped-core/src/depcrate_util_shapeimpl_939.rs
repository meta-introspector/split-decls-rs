// Generated macro for impl_939 (impl)
macro_rules! Depcrate_util_shapeimpl_939 {
() => {
// Module: crate::util::shape
// Provides: {"impl_939"}
// Dependencies: {}
impl FromIterator < Shape > for ShapeSet { fn from_iter < T : IntoIterator < Item = Shape > > (iter : T) -> Self { let mut output = ShapeSet :: default () ; for shape in iter . into_iter () { output . insert (shape) ; } output } }
};
}
