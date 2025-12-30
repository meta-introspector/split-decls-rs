// Generated macro for impl_196 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_196 {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_196"}
// Dependencies: {}
impl < V : LogScalable > ReversibleRanged for LogCoord < V > { fn unmap (& self , input : i32 , limit : (i32 , i32)) -> Option < V > { self . linear . unmap (input , limit) . map (| value_ln | { let fv = value_ln . exp () ; self . f64_to_value (fv) }) } }
};
}
