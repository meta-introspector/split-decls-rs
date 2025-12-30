// Generated macro for macro_283 (macro)
macro_rules! Depcrate_drop_forget_uselessmacro_283 {
() => {
// Module: crate::drop_forget_useless
// Provides: {"macro_283"}
// Dependencies: {}
declare_lint ! { # [doc = " The `undropped_manually_drops` lint check for calls to `std::mem::drop` with"] # [doc = " a value of `std::mem::ManuallyDrop` which doesn't drop."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " struct S;"] # [doc = " drop(std::mem::ManuallyDrop::new(S));"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " `ManuallyDrop` does not drop it's inner value so calling `std::mem::drop` will"] # [doc = " not drop the inner value of the `ManuallyDrop` either."] pub UNDROPPED_MANUALLY_DROPS , Deny , "calls to `std::mem::drop` with `std::mem::ManuallyDrop` instead of it's inner value" }
};
}
