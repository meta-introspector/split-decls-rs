// Generated macro for impl_597 (impl)
macro_rules! Depcrate_stats_univariate_kdeimpl_597 {
() => {
// Module: crate::stats::univariate::kde
// Provides: {"impl_597"}
// Dependencies: {}
impl Bandwidth { fn estimate < A : Float > (self , sample : & Sample < A >) -> A { match self { Bandwidth :: Silverman => { let factor = A :: cast (4. / 3.) ; let exponent = A :: cast (1. / 5.) ; let n = A :: cast (sample . len ()) ; let sigma = sample . std_dev (None) ; sigma * (factor / n) . powf (exponent) } } } }
};
}
