// Generated macro for macro_7050 (macro)
macro_rules! Depcrate_methodsmacro_7050 {
() => {
// Module: crate::methods
// Provides: {"macro_7050"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " When sorting primitive values (integers, bools, chars, as well"] # [doc = " as arrays, slices, and tuples of such items), it is typically better to"] # [doc = " use an unstable sort than a stable sort."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Typically, using a stable sort consumes more memory and cpu cycles."] # [doc = " Because values which compare equal are identical, preserving their"] # [doc = " relative order (the guarantee that a stable sort provides) means"] # [doc = " nothing, while the extra costs still apply."] # [doc = ""] # [doc = " ### Known problems"] # [doc = ""] # [doc = " As pointed out in"] # [doc = " [issue #8241](https://github.com/rust-lang/rust-clippy/issues/8241),"] # [doc = " a stable sort can instead be significantly faster for certain scenarios"] # [doc = " (eg. when a sorted vector is extended with new data and resorted)."] # [doc = ""] # [doc = " For more information and benchmarking results, please refer to the"] # [doc = " issue linked above."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut vec = vec![2, 1, 3];"] # [doc = " vec.sort();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut vec = vec![2, 1, 3];"] # [doc = " vec.sort_unstable();"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub STABLE_SORT_PRIMITIVE , pedantic , "use of sort() when sort_unstable() is equivalent" }
};
}
