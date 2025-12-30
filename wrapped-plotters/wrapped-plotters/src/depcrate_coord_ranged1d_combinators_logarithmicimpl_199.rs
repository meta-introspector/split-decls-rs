// Generated macro for impl_199 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_199 {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_199"}
// Dependencies: {}
impl < V : LogScalable > From < LogRangeExt < V > > for LogCoord < V > { fn from (spec : LogRangeExt < V >) -> LogCoord < V > { let zero_point = spec . zero ; let mut start = spec . range . start . as_f64 () - zero_point ; let mut end = spec . range . end . as_f64 () - zero_point ; let negative = if start < 0.0 || end < 0.0 { start = - start ; end = - end ; true } else { false } ; if start < end { if start == 0.0 { start = start . max (end * 1e-5) ; } } else if end == 0.0 { end = end . max (start * 1e-5) ; } LogCoord { linear : (start . ln () .. end . ln ()) . into () , logic : spec . range , normalized : start .. end , base : spec . base , zero_point , negative , marker : PhantomData , } } }
};
}
