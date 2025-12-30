// Generated macro for impl_202 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_202 {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_202"}
// Dependencies: {}
impl < V : LogScalable > LogCoord < V > { fn value_to_f64 (& self , value : & V) -> f64 { let fv = value . as_f64 () - self . zero_point ; if self . negative { - fv } else { fv } } fn f64_to_value (& self , fv : f64) -> V { let fv = if self . negative { - fv } else { fv } ; V :: from_f64 (fv + self . zero_point) } fn is_inf (& self , fv : f64) -> bool { let fv = if self . negative { - fv } else { fv } ; let a = V :: from_f64 (fv + self . zero_point) ; let b = V :: from_f64 (self . zero_point) ; (V :: as_f64 (& a) - V :: as_f64 (& b)) . abs () < f64 :: EPSILON } }
};
}
