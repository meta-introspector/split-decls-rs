// Generated macro for LogCoord (struct)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicLogCoord {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"LogCoord"}
// Dependencies: {}
# [doc = " A log scaled coordinate axis"] # [derive (Clone)] pub struct LogCoord < V : LogScalable > { linear : RangedCoordf64 , logic : Range < V > , normalized : Range < f64 > , base : f64 , zero_point : f64 , negative : bool , marker : PhantomData < V > , }
};
}
