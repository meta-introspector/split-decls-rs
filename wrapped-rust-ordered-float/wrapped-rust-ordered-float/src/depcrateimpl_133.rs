// Generated macro for impl_133 (impl)
macro_rules! Depcrateimpl_133 {
() => {
// Module: crate
// Provides: {"impl_133"}
// Dependencies: {}
impl NotNan < f64 > { # [doc = " Converts this [`NotNan`]`<`[`f64`]`>` to a [`NotNan`]`<`[`f32`]`>` while giving up on"] # [doc = " precision, [using `roundTiesToEven` as rounding mode, yielding `Infinity` on"] # [doc = " overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#semantics)."] # [doc = ""] # [doc = " Note: For the reverse conversion (from `NotNan<f32>` to `NotNan<f64>`), you can use"] # [doc = " `.into()`."] pub fn as_f32 (self) -> NotNan < f32 > { NotNan (self . 0 as f32) } }
};
}
