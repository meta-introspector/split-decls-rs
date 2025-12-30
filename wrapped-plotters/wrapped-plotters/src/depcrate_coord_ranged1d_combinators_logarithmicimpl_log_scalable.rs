// Generated macro for impl_log_scalable (macro)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_log_scalable {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_log_scalable"}
// Dependencies: {}
macro_rules ! impl_log_scalable { (i , $ t : ty) => { impl LogScalable for $ t { fn as_f64 (& self) -> f64 { if * self != 0 { return * self as f64 ; } return 0.5 ; } fn from_f64 (f : f64) -> $ t { f . round () as $ t } } } ; (f , $ t : ty) => { impl LogScalable for $ t { fn as_f64 (& self) -> f64 { * self as f64 } fn from_f64 (f : f64) -> $ t { f as $ t } } } ; }
};
}
