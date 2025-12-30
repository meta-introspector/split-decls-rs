// Generated macro for macro_4968 (macro)
macro_rules! Depcrate_matchesmacro_4968 {
() => {
// Module: crate::matches
// Provides: {"macro_4968"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for matches with two arms where an `if let else` will"] # [doc = " usually suffice."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Just readability – `if let` nests less than a `match`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Personal style preferences may differ."] # [doc = ""] # [doc = " ### Example"] # [doc = " Using `match`:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn bar(foo: &usize) {}"] # [doc = " # let other_ref: usize = 1;"] # [doc = " # let x: Option<&usize> = Some(&1);"] # [doc = " match x {"] # [doc = "     Some(ref foo) => bar(foo),"] # [doc = "     _ => bar(&other_ref),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Using `if let` with `else`:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn bar(foo: &usize) {}"] # [doc = " # let other_ref: usize = 1;"] # [doc = " # let x: Option<&usize> = Some(&1);"] # [doc = " if let Some(ref foo) = x {"] # [doc = "     bar(foo);"] # [doc = " } else {"] # [doc = "     bar(&other_ref);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SINGLE_MATCH_ELSE , pedantic , "a `match` statement with two arms where the second arm's pattern is a placeholder instead of a specific match pattern" }
};
}
