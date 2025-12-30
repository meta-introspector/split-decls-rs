// Generated macro for macro_8935 (macro)
macro_rules! Depcrate_rangesmacro_8935 {
() => {
// Module: crate::ranges
// Provides: {"macro_8935"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for range expressions `x..y` where both `x` and `y`"] # [doc = " are constant and `x` is greater to `y`. Also triggers if `x` is equal to `y` when they are conditions to a `for` loop."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Empty ranges yield no values so iterating them is a no-op."] # [doc = " Moreover, trying to use a reversed range to index a slice will panic at run-time."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " fn main() {"] # [doc = "     (10..=0).for_each(|x| println!(\"{}\", x));"] # [doc = ""] # [doc = "     let arr = [1, 2, 3, 4, 5];"] # [doc = "     let sub = &arr[3..1];"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     (0..=10).rev().for_each(|x| println!(\"{}\", x));"] # [doc = ""] # [doc = "     let arr = [1, 2, 3, 4, 5];"] # [doc = "     let sub = &arr[1..3];"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub REVERSED_EMPTY_RANGES , correctness , "reversing the limits of range expressions, resulting in empty ranges" }
};
}
