// Generated macro for macro_1359 (macro)
macro_rules! Depcrate_unusedmacro_1359 {
() => {
// Module: crate::unused
// Provides: {"macro_1359"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unused_allocation` lint detects unnecessary allocations that can"] # [doc = " be eliminated."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn main() {"] # [doc = "     let a = Box::new([1, 2, 3]).len();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " When a `box` expression is immediately coerced to a reference, then"] # [doc = " the allocation is unnecessary, and a reference (using `&` or `&mut`)"] # [doc = " should be used instead to avoid the allocation."] pub (super) UNUSED_ALLOCATION , Warn , "detects unnecessary allocations that can be eliminated" }
};
}
