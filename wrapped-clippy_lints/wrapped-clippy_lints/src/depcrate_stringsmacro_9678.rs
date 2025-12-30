// Generated macro for macro_9678 (macro)
macro_rules! Depcrate_stringsmacro_9678 {
() => {
// Module: crate::strings
// Provides: {"macro_9678"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for slice operations on strings"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " UTF-8 characters span multiple bytes, and it is easy to inadvertently confuse character"] # [doc = " counts and string indices. This may lead to panics, and should warrant some test cases"] # [doc = " containing wide UTF-8 characters. This lint is most useful in code that should avoid"] # [doc = " panics at all costs."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Probably lots of false positives. If an index comes from a known valid position (e.g."] # [doc = " obtained via `char_indices` over the same string), it is totally OK."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,should_panic"] # [doc = " &\"Ölkanne\"[1..];"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub STRING_SLICE , restriction , "slicing a string" }
};
}
