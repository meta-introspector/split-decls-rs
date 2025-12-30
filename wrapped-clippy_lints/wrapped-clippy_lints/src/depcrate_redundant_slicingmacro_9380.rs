// Generated macro for macro_9380 (macro)
macro_rules! Depcrate_redundant_slicingmacro_9380 {
() => {
// Module: crate::redundant_slicing
// Provides: {"macro_9380"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for redundant slicing expressions which use the full range, and"] # [doc = " do not change the type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It unnecessarily adds complexity to the expression."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the type being sliced has an implementation of `Index<RangeFull>`"] # [doc = " that actually changes anything then it can't be removed. However, this would be surprising"] # [doc = " to people reading the code and should have a note with it."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " fn get_slice(x: &[u32]) -> &[u32] {"] # [doc = "     &x[..]"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " fn get_slice(x: &[u32]) -> &[u32] {"] # [doc = "     x"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub REDUNDANT_SLICING , complexity , "redundant slicing of the whole range of a type" }
};
}
