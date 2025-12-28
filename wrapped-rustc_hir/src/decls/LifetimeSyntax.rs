macro_rules! LifetimeSyntax {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , HashStable_Generic)] pub enum LifetimeSyntax { # [doc = " E.g. `&Type`, `ContainsLifetime`"] Implicit , # [doc = " E.g. `&'_ Type`, `ContainsLifetime<'_>`, `impl Trait + '_`, `impl Trait + use<'_>`"] ExplicitAnonymous , # [doc = " E.g. `&'a Type`, `ContainsLifetime<'a>`, `impl Trait + 'a`, `impl Trait + use<'a>`"] ExplicitBound , }
    };
}

LifetimeSyntax!();