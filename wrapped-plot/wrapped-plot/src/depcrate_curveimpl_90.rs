// Generated macro for impl_90 (impl)
macro_rules! Depcrate_curveimpl_90 {
() => {
// Module: crate::curve
// Provides: {"impl_90"}
// Dependencies: {}
impl < X , Y > Curve < X , Y > { fn style (& self) -> Style { match * self { Curve :: Dots { .. } => Style :: Dots , Curve :: Impulses { .. } => Style :: Impulses , Curve :: Lines { .. } => Style :: Lines , Curve :: LinesPoints { .. } => Style :: LinesPoints , Curve :: Points { .. } => Style :: Points , Curve :: Steps { .. } => Style :: Steps , } } }
};
}
