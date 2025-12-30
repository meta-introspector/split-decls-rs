// Generated macro for macro_3947 (macro)
macro_rules! Depcrate_loopsmacro_3947 {
() => {
// Module: crate::loops
// Provides: {"macro_3947"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks `for` loops over slices with an explicit counter"] # [doc = " and suggests the use of `.enumerate()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `.enumerate()` makes the intent more clear,"] # [doc = " declutters the code and may be faster in some instances."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let v = vec![1];"] # [doc = " # fn bar(bar: usize, baz: usize) {}"] # [doc = " let mut i = 0;"] # [doc = " for item in &v {"] # [doc = "     bar(i, *item);"] # [doc = "     i += 1;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let v = vec![1];"] # [doc = " # fn bar(bar: usize, baz: usize) {}"] # [doc = " for (i, item) in v.iter().enumerate() { bar(i, *item); }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXPLICIT_COUNTER_LOOP , complexity , "for-looping with an explicit counter when `_.enumerate()` would do" }
};
}
