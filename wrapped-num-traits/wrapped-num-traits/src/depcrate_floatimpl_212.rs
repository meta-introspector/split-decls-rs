// Generated macro for impl_212 (impl)
macro_rules! Depcrate_floatimpl_212 {
() => {
// Module: crate::float
// Provides: {"impl_212"}
// Dependencies: {}
impl FloatCore for f64 { constant ! { infinity () -> f64 :: INFINITY ; neg_infinity () -> f64 :: NEG_INFINITY ; nan () -> f64 :: NAN ; neg_zero () -> - 0.0 ; min_value () -> f64 :: MIN ; min_positive_value () -> f64 :: MIN_POSITIVE ; epsilon () -> f64 :: EPSILON ; max_value () -> f64 :: MAX ; } # [inline] fn integer_decode (self) -> (u64 , i16 , i8) { integer_decode_f64 (self) } forward ! { Self :: is_nan (self) -> bool ; Self :: is_infinite (self) -> bool ; Self :: is_finite (self) -> bool ; Self :: is_normal (self) -> bool ; Self :: is_subnormal (self) -> bool ; Self :: clamp (self , min : Self , max : Self) -> Self ; Self :: classify (self) -> FpCategory ; Self :: is_sign_positive (self) -> bool ; Self :: is_sign_negative (self) -> bool ; Self :: min (self , other : Self) -> Self ; Self :: max (self , other : Self) -> Self ; Self :: recip (self) -> Self ; Self :: to_degrees (self) -> Self ; Self :: to_radians (self) -> Self ; } # [cfg (feature = "std")] forward ! { Self :: floor (self) -> Self ; Self :: ceil (self) -> Self ; Self :: round (self) -> Self ; Self :: trunc (self) -> Self ; Self :: fract (self) -> Self ; Self :: abs (self) -> Self ; Self :: signum (self) -> Self ; Self :: powi (self , n : i32) -> Self ; } # [cfg (all (not (feature = "std") , feature = "libm"))] forward ! { libm :: floor as floor (self) -> Self ; libm :: ceil as ceil (self) -> Self ; libm :: round as round (self) -> Self ; libm :: trunc as trunc (self) -> Self ; libm :: fabs as abs (self) -> Self ; } # [cfg (all (not (feature = "std") , feature = "libm"))] # [inline] fn fract (self) -> Self { self - libm :: trunc (self) } }
};
}
