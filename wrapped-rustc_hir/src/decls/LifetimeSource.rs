macro_rules! deps {
    () => {
        Path!();
        AngleBrackets!();
    };
}

macro_rules! LifetimeSource {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , HashStable_Generic)] pub enum LifetimeSource { # [doc = " E.g. `&Type`, `&'_ Type`, `&'a Type`, `&mut Type`, `&'_ mut Type`, `&'a mut Type`"] Reference , # [doc = " E.g. `ContainsLifetime`, `ContainsLifetime<>`, `ContainsLifetime<'_>`,"] # [doc = " `ContainsLifetime<'a>`"] Path { angle_brackets : AngleBrackets } , # [doc = " E.g. `impl Trait + '_`, `impl Trait + 'a`"] OutlivesBound , # [doc = " E.g. `impl Trait + use<'_>`, `impl Trait + use<'a>`"] PreciseCapturing , # [doc = " Other usages which have not yet been categorized. Feel free to"] # [doc = " add new sources that you find useful."] # [doc = ""] # [doc = " Some non-exhaustive examples:"] # [doc = " - `where T: 'a`"] # [doc = " - `fn(_: dyn Trait + 'a)`"] Other , }
    };
}

LifetimeSource!();