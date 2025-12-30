// Generated macro for float_impl_libm (macro)
macro_rules! Depcrate_floatfloat_impl_libm {
() => {
// Module: crate::float
// Provides: {"float_impl_libm"}
// Dependencies: {}
# [cfg (all (not (feature = "std") , feature = "libm"))] macro_rules ! float_impl_libm { ($ T : ident $ decode : ident) => { constant ! { nan () -> $ T :: NAN ; infinity () -> $ T :: INFINITY ; neg_infinity () -> $ T :: NEG_INFINITY ; neg_zero () -> - 0.0 ; min_value () -> $ T :: MIN ; min_positive_value () -> $ T :: MIN_POSITIVE ; epsilon () -> $ T :: EPSILON ; max_value () -> $ T :: MAX ; } # [inline] fn integer_decode (self) -> (u64 , i16 , i8) { $ decode (self) } # [inline] fn fract (self) -> Self { self - Float :: trunc (self) } # [inline] fn log (self , base : Self) -> Self { self . ln () / base . ln () } forward ! { Self :: is_nan (self) -> bool ; Self :: is_infinite (self) -> bool ; Self :: is_finite (self) -> bool ; Self :: is_normal (self) -> bool ; Self :: is_subnormal (self) -> bool ; Self :: clamp (self , min : Self , max : Self) -> Self ; Self :: classify (self) -> FpCategory ; Self :: is_sign_positive (self) -> bool ; Self :: is_sign_negative (self) -> bool ; Self :: min (self , other : Self) -> Self ; Self :: max (self , other : Self) -> Self ; Self :: recip (self) -> Self ; Self :: to_degrees (self) -> Self ; Self :: to_radians (self) -> Self ; } forward ! { FloatCore :: signum (self) -> Self ; FloatCore :: powi (self , n : i32) -> Self ; } } ; }
};
}
