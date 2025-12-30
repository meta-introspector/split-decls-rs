// Generated macro for macro_7219 (macro)
macro_rules! Depcrate_methodsmacro_7219 {
() => {
// Module: crate::methods
// Provides: {"macro_7219"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for iterator combinator calls such as `.take(x)` or `.skip(x)`"] # [doc = " where `x` is greater than the amount of items that an iterator will produce."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Taking or skipping more items than there are in an iterator either creates an iterator"] # [doc = " with all items from the original iterator or an iterator with no items at all."] # [doc = " This is most likely not what the user intended to do."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " for _ in [1, 2, 3].iter().take(4) {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " for _ in [1, 2, 3].iter() {}"] # [doc = " ```"] # [clippy :: version = "1.74.0"] pub ITER_OUT_OF_BOUNDS , suspicious , "calls to `.take()` or `.skip()` that are out of bounds" }
};
}
