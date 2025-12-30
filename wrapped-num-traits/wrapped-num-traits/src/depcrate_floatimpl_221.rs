// Generated macro for impl_221 (impl)
macro_rules! Depcrate_floatimpl_221 {
() => {
// Module: crate::float
// Provides: {"impl_221"}
// Dependencies: {}
# [cfg (all (not (feature = "std") , feature = "libm"))] impl Float for f64 { float_impl_libm ! (f64 integer_decode_f64) ; # [inline] # [allow (deprecated)] fn abs_sub (self , other : Self) -> Self { libm :: fdim (self , other) } forward ! { libm :: floor as floor (self) -> Self ; libm :: ceil as ceil (self) -> Self ; libm :: round as round (self) -> Self ; libm :: trunc as trunc (self) -> Self ; libm :: fabs as abs (self) -> Self ; libm :: fma as mul_add (self , a : Self , b : Self) -> Self ; libm :: pow as powf (self , n : Self) -> Self ; libm :: sqrt as sqrt (self) -> Self ; libm :: exp as exp (self) -> Self ; libm :: exp2 as exp2 (self) -> Self ; libm :: log as ln (self) -> Self ; libm :: log2 as log2 (self) -> Self ; libm :: log10 as log10 (self) -> Self ; libm :: cbrt as cbrt (self) -> Self ; libm :: hypot as hypot (self , other : Self) -> Self ; libm :: sin as sin (self) -> Self ; libm :: cos as cos (self) -> Self ; libm :: tan as tan (self) -> Self ; libm :: asin as asin (self) -> Self ; libm :: acos as acos (self) -> Self ; libm :: atan as atan (self) -> Self ; libm :: atan2 as atan2 (self , other : Self) -> Self ; libm :: sincos as sin_cos (self) -> (Self , Self) ; libm :: expm1 as exp_m1 (self) -> Self ; libm :: log1p as ln_1p (self) -> Self ; libm :: sinh as sinh (self) -> Self ; libm :: cosh as cosh (self) -> Self ; libm :: tanh as tanh (self) -> Self ; libm :: asinh as asinh (self) -> Self ; libm :: acosh as acosh (self) -> Self ; libm :: atanh as atanh (self) -> Self ; libm :: copysign as copysign (self , sign : Self) -> Self ; } }
};
}
