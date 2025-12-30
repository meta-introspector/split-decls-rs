// Generated macro for impl_220 (impl)
macro_rules! Depcrate_floatimpl_220 {
() => {
// Module: crate::float
// Provides: {"impl_220"}
// Dependencies: {}
# [cfg (all (not (feature = "std") , feature = "libm"))] impl Float for f32 { float_impl_libm ! (f32 integer_decode_f32) ; # [inline] # [allow (deprecated)] fn abs_sub (self , other : Self) -> Self { libm :: fdimf (self , other) } forward ! { libm :: floorf as floor (self) -> Self ; libm :: ceilf as ceil (self) -> Self ; libm :: roundf as round (self) -> Self ; libm :: truncf as trunc (self) -> Self ; libm :: fabsf as abs (self) -> Self ; libm :: fmaf as mul_add (self , a : Self , b : Self) -> Self ; libm :: powf as powf (self , n : Self) -> Self ; libm :: sqrtf as sqrt (self) -> Self ; libm :: expf as exp (self) -> Self ; libm :: exp2f as exp2 (self) -> Self ; libm :: logf as ln (self) -> Self ; libm :: log2f as log2 (self) -> Self ; libm :: log10f as log10 (self) -> Self ; libm :: cbrtf as cbrt (self) -> Self ; libm :: hypotf as hypot (self , other : Self) -> Self ; libm :: sinf as sin (self) -> Self ; libm :: cosf as cos (self) -> Self ; libm :: tanf as tan (self) -> Self ; libm :: asinf as asin (self) -> Self ; libm :: acosf as acos (self) -> Self ; libm :: atanf as atan (self) -> Self ; libm :: atan2f as atan2 (self , other : Self) -> Self ; libm :: sincosf as sin_cos (self) -> (Self , Self) ; libm :: expm1f as exp_m1 (self) -> Self ; libm :: log1pf as ln_1p (self) -> Self ; libm :: sinhf as sinh (self) -> Self ; libm :: coshf as cosh (self) -> Self ; libm :: tanhf as tanh (self) -> Self ; libm :: asinhf as asinh (self) -> Self ; libm :: acoshf as acosh (self) -> Self ; libm :: atanhf as atanh (self) -> Self ; libm :: copysignf as copysign (self , other : Self) -> Self ; } }
};
}
