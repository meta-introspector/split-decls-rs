// Generated macro for macro_5029 (macro)
macro_rules! Depcrate_matchesmacro_5029 {
() => {
// Module: crate::matches
// Provides: {"macro_5029"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary '..' pattern binding on struct when all fields are explicitly matched."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Correctness and readability. It's like having a wildcard pattern after"] # [doc = " matching all enum variants explicitly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # struct A { a: i32 }"] # [doc = " let a = A { a: 5 };"] # [doc = ""] # [doc = " match a {"] # [doc = "     A { a: 5, .. } => {},"] # [doc = "     _ => {},"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # struct A { a: i32 }"] # [doc = " # let a = A { a: 5 };"] # [doc = " match a {"] # [doc = "     A { a: 5 } => {},"] # [doc = "     _ => {},"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub REST_PAT_IN_FULLY_BOUND_STRUCTS , restriction , "a match on a struct that binds all fields but still uses the wildcard pattern" }
};
}
