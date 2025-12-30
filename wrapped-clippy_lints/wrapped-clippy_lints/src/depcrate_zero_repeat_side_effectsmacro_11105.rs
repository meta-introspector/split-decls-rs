// Generated macro for macro_11105 (macro)
macro_rules! Depcrate_zero_repeat_side_effectsmacro_11105 {
() => {
// Module: crate::zero_repeat_side_effects
// Provides: {"macro_11105"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for array or vec initializations which call a function or method,"] # [doc = " but which have a repeat count of zero."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Such an initialization, despite having a repeat length of 0, will still call the inner function."] # [doc = " This may not be obvious and as such there may be unintended side effects in code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn side_effect() -> i32 {"] # [doc = "     println!(\"side effect\");"] # [doc = "     10"] # [doc = " }"] # [doc = " let a = [side_effect(); 0];"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn side_effect() -> i32 {"] # [doc = "     println!(\"side effect\");"] # [doc = "     10"] # [doc = " }"] # [doc = " side_effect();"] # [doc = " let a: [i32; 0] = [];"] # [doc = " ```"] # [clippy :: version = "1.79.0"] pub ZERO_REPEAT_SIDE_EFFECTS , suspicious , "usage of zero-sized initializations of arrays or vecs causing side effects" }
};
}
