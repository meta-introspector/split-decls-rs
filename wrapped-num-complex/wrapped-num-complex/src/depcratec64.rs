// Generated macro for c64 (function)
macro_rules! Depcratec64 {
() => {
// Module: crate
// Provides: {"c64"}
// Dependencies: {}
# [doc = " Create a new [`Complex<f64>`] with arguments that can convert [`Into<f64>`]."] # [doc = ""] # [doc = " ```"] # [doc = " use num_complex::{c64, Complex64};"] # [doc = " assert_eq!(c64(1, 2), Complex64::new(1.0, 2.0));"] # [doc = " ```"] # [inline] pub fn c64 < T : Into < f64 > > (re : T , im : T) -> Complex64 { Complex :: new (re . into () , im . into ()) }
};
}
