// Generated macro for macro_8731 (macro)
macro_rules! Depcrate_operatorsmacro_8731 {
() => {
// Module: crate::operators
// Provides: {"macro_8731"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for comparisons where one side of the relation is"] # [doc = " either the minimum or maximum value for its type and warns if it involves a"] # [doc = " case that is always true or always false. Only integer and boolean types are"] # [doc = " checked."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " An expression like `min <= x` may misleadingly imply"] # [doc = " that it is possible for `x` to be less than the minimum. Expressions like"] # [doc = " `max < x` are probably mistakes."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " For `usize` the size of the current compile target will"] # [doc = " be assumed (e.g., 64 bits on 64 bit systems). This means code that uses such"] # [doc = " a comparison to detect target pointer width will trigger this lint. One can"] # [doc = " use `mem::sizeof` and compare its value or conditional compilation"] # [doc = " attributes"] # [doc = " like `#[cfg(target_pointer_width = \"64\")] ..` instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let vec: Vec<isize> = Vec::new();"] # [doc = " if vec.len() <= 0 {}"] # [doc = " if 100 > i32::MAX {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ABSURD_EXTREME_COMPARISONS , correctness , "a comparison with a maximum or minimum value that is always true or false" }
};
}
