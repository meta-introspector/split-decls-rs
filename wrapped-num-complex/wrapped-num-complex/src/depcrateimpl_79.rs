// Generated macro for impl_79 (impl)
macro_rules! Depcrateimpl_79 {
() => {
// Module: crate
// Provides: {"impl_79"}
// Dependencies: {}
impl < T : FloatCore > Complex < T > { # [doc = " Checks if the given complex number is NaN"] # [inline] pub fn is_nan (self) -> bool { self . re . is_nan () || self . im . is_nan () } # [doc = " Checks if the given complex number is infinite"] # [inline] pub fn is_infinite (self) -> bool { ! self . is_nan () && (self . re . is_infinite () || self . im . is_infinite ()) } # [doc = " Checks if the given complex number is finite"] # [inline] pub fn is_finite (self) -> bool { self . re . is_finite () && self . im . is_finite () } # [doc = " Checks if the given complex number is normal"] # [inline] pub fn is_normal (self) -> bool { self . re . is_normal () && self . im . is_normal () } }
};
}
