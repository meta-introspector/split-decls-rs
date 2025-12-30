// Generated macro for macro_4975 (macro)
macro_rules! Depcrate_matchesmacro_4975 {
() => {
// Module: crate::matches
// Provides: {"macro_4975"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for wildcard enum matches for a single variant."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " New enum variants added by library updates can be missed."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Suggested replacements may not use correct path to enum"] # [doc = " if it's not present in the current scope."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # enum Foo { A, B, C }"] # [doc = " # let x = Foo::B;"] # [doc = " match x {"] # [doc = "     Foo::A => {},"] # [doc = "     Foo::B => {},"] # [doc = "     _ => {},"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # enum Foo { A, B, C }"] # [doc = " # let x = Foo::B;"] # [doc = " match x {"] # [doc = "     Foo::A => {},"] # [doc = "     Foo::B => {},"] # [doc = "     Foo::C => {},"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub MATCH_WILDCARD_FOR_SINGLE_VARIANTS , pedantic , "a wildcard enum match for a single variant" }
};
}
