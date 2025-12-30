// Generated macro for macro_4974 (macro)
macro_rules! Depcrate_matchesmacro_4974 {
() => {
// Module: crate::matches
// Provides: {"macro_4974"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for wildcard enum matches using `_`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " New enum variants added by library updates can be missed."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Suggested replacements may be incorrect if guards exhaustively cover some"] # [doc = " variants, and also may not use correct path to enum if it's not present in the current scope."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # enum Foo { A(usize), B(usize) }"] # [doc = " # let x = Foo::B(1);"] # [doc = " match x {"] # [doc = "     Foo::A(_) => {},"] # [doc = "     _ => {},"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # enum Foo { A(usize), B(usize) }"] # [doc = " # let x = Foo::B(1);"] # [doc = " match x {"] # [doc = "     Foo::A(_) => {},"] # [doc = "     Foo::B(_) => {},"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.34.0"] pub WILDCARD_ENUM_MATCH_ARM , restriction , "a wildcard enum match arm using `_`" }
};
}
