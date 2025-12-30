// Generated macro for macro_4405 (macro)
macro_rules! Depcrate_manual_slice_size_calculationmacro_4405 {
() => {
// Module: crate::manual_slice_size_calculation
// Provides: {"macro_4405"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " When `a` is `&[T]`, detect `a.len() * size_of::<T>()` and suggest `size_of_val(a)`"] # [doc = " instead."] # [doc = ""] # [doc = " ### Why is this better?"] # [doc = " * Shorter to write"] # [doc = " * Removes the need for the human and the compiler to worry about overflow in the"] # [doc = "   multiplication"] # [doc = " * Potentially faster at runtime as rust emits special no-wrapping flags when it"] # [doc = "   calculates the byte length"] # [doc = " * Less turbofishing"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let data : &[i32] = &[1, 2, 3];"] # [doc = " let newlen = data.len() * size_of::<i32>();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let data : &[i32] = &[1, 2, 3];"] # [doc = " let newlen = size_of_val(data);"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub MANUAL_SLICE_SIZE_CALCULATION , complexity , "manual slice size calculation" }
};
}
