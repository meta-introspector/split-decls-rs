macro_rules! deps {
    () => {
        LifetimeSyntax!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl From < Ident > for LifetimeSyntax { fn from (ident : Ident) -> Self { let name = ident . name ; if name == sym :: empty { unreachable ! ("A lifetime name should never be empty") ; } else if name == kw :: UnderscoreLifetime { LifetimeSyntax :: ExplicitAnonymous } else { debug_assert ! (name . as_str () . starts_with ('\'')) ; LifetimeSyntax :: ExplicitBound } } }
    };
}

impl_106!();