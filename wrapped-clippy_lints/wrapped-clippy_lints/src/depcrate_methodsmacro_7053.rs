// Generated macro for macro_7053 (macro)
macro_rules! Depcrate_methodsmacro_7053 {
() => {
// Module: crate::methods
// Provides: {"macro_7053"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Vec::sort_by` passing in a closure"] # [doc = " which compares the two arguments, either directly or indirectly."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is more clear to use `Vec::sort_by_key` (or `Vec::sort` if"] # [doc = " possible) than to use `Vec::sort_by` and a more complicated"] # [doc = " closure."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the suggested `Vec::sort_by_key` uses Reverse and it isn't already"] # [doc = " imported by a use statement, then it will need to be added manually."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # struct A;"] # [doc = " # impl A { fn foo(&self) {} }"] # [doc = " # let mut vec: Vec<A> = Vec::new();"] # [doc = " vec.sort_by(|a, b| a.foo().cmp(&b.foo()));"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # struct A;"] # [doc = " # impl A { fn foo(&self) {} }"] # [doc = " # let mut vec: Vec<A> = Vec::new();"] # [doc = " vec.sort_by_key(|a| a.foo());"] # [doc = " ```"] # [clippy :: version = "1.46.0"] pub UNNECESSARY_SORT_BY , complexity , "Use of `Vec::sort_by` when `Vec::sort_by_key` or `Vec::sort` would be clearer" }
};
}
