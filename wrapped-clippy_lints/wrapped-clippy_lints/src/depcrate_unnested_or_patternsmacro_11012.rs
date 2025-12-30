// Generated macro for macro_11012 (macro)
macro_rules! Depcrate_unnested_or_patternsmacro_11012 {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"macro_11012"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnested or-patterns, e.g., `Some(0) | Some(2)` and"] # [doc = " suggests replacing the pattern with a nested one, `Some(0 | 2)`."] # [doc = ""] # [doc = " Another way to think of this is that it rewrites patterns in"] # [doc = " *disjunctive normal form (DNF)* into *conjunctive normal form (CNF)*."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In the example above, `Some` is repeated, which unnecessarily complicates the pattern."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     if let Some(0) | Some(2) = Some(0) {}"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     if let Some(0 | 2) = Some(0) {}"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.46.0"] pub UNNESTED_OR_PATTERNS , pedantic , "unnested or-patterns, e.g., `Foo(Bar) | Foo(Baz) instead of `Foo(Bar | Baz)`" }
};
}
