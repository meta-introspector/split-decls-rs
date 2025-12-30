// Generated macro for impl_48 (impl)
macro_rules! Depcrate_complex_floatimpl_48 {
() => {
// Module: crate::complex_float
// Provides: {"impl_48"}
// Dependencies: {}
impl < T > ComplexFloat for T where T : Float + FloatConst , { type Real = T ; fn re (self) -> Self :: Real { self } fn im (self) -> Self :: Real { T :: zero () } fn l1_norm (& self) -> Self :: Real { self . abs () } fn arg (self) -> Self :: Real { if self . is_nan () { self } else if self . is_sign_negative () { T :: PI () } else { T :: zero () } } fn powc (self , exp : Complex < Self :: Real >) -> Complex < Self :: Real > { Complex :: new (self , T :: zero ()) . powc (exp) } fn conj (self) -> Self { self } fn expf (self , base : Self :: Real) -> Self { base . powf (self) } forward ! { Float :: is_normal (self) -> bool ; Float :: is_infinite (self) -> bool ; Float :: is_finite (self) -> bool ; Float :: is_nan (self) -> bool ; Float :: recip (self) -> Self ; Float :: powi (self , n : i32) -> Self ; Float :: powf (self , f : Self) -> Self ; Float :: sqrt (self) -> Self ; Float :: cbrt (self) -> Self ; Float :: exp (self) -> Self ; Float :: exp2 (self) -> Self ; Float :: ln (self) -> Self ; Float :: log (self , base : Self) -> Self ; Float :: log2 (self) -> Self ; Float :: log10 (self) -> Self ; Float :: sin (self) -> Self ; Float :: cos (self) -> Self ; Float :: tan (self) -> Self ; Float :: asin (self) -> Self ; Float :: acos (self) -> Self ; Float :: atan (self) -> Self ; Float :: sinh (self) -> Self ; Float :: cosh (self) -> Self ; Float :: tanh (self) -> Self ; Float :: asinh (self) -> Self ; Float :: acosh (self) -> Self ; Float :: atanh (self) -> Self ; Float :: abs (self) -> Self ; } }
};
}
