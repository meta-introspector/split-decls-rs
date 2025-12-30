// Generated macro for c32 (function)
macro_rules! Depcratec32 {
() => {
// Module: crate
// Provides: {"c32"}
// Dependencies: {}
# [doc = " Create a new [`Complex<f32>`] with arguments that can convert [`Into<f32>`]."] # [doc = ""] # [doc = " ```"] # [doc = " use num_complex::{c32, Complex32};"] # [doc = " assert_eq!(c32(1u8, 2), Complex32::new(1.0, 2.0));"] # [doc = " ```"] # [doc = ""] # [doc = " Note: ambiguous integer literals in Rust will [default] to `i32`, which does **not** implement"] # [doc = " `Into<f32>`, so a call like `c32(1, 2)` will result in a type error. The example above uses a"] # [doc = " suffixed `1u8` to set its type, and then the `2` can be inferred as the same type."] # [doc = ""] # [doc = " [default]: https://doc.rust-lang.org/reference/expressions/literal-expr.html#integer-literal-expressions"] # [inline] pub fn c32 < T : Into < f32 > > (re : T , im : T) -> Complex32 { Complex :: new (re . into () , im . into ()) }
};
}
