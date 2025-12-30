// Generated macro for LifetimeSyntax (enum)
macro_rules! Depcrate_hirLifetimeSyntax {
() => {
// Module: crate::hir
// Provides: {"LifetimeSyntax"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq , HashStable_Generic)] pub enum LifetimeSyntax { # [doc = " E.g. `&Type`, `ContainsLifetime`"] Implicit , # [doc = " E.g. `&'_ Type`, `ContainsLifetime<'_>`, `impl Trait + '_`, `impl Trait + use<'_>`"] ExplicitAnonymous , # [doc = " E.g. `&'a Type`, `ContainsLifetime<'a>`, `impl Trait + 'a`, `impl Trait + use<'a>`"] ExplicitBound , }
};
}
