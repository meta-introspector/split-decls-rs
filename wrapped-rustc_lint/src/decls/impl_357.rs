macro_rules! deps {
    () => {
        LifetimeSyntaxCategory!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl LifetimeSyntaxCategory { fn new (syntax_source : (hir :: LifetimeSyntax , LifetimeSource)) -> Option < Self > { use LifetimeSource :: * ; use hir :: LifetimeSyntax :: * ; match syntax_source { (Implicit , Reference) | (ExplicitAnonymous , Reference) | (ExplicitAnonymous , Path { .. }) | (ExplicitAnonymous , OutlivesBound | PreciseCapturing) => { Some (Self :: Elided) } (Implicit , Path { .. }) => { Some (Self :: Hidden) } (ExplicitBound , Reference) | (ExplicitBound , Path { .. }) | (ExplicitBound , OutlivesBound | PreciseCapturing) => { Some (Self :: Named) } (Implicit , OutlivesBound | PreciseCapturing) | (_ , Other) => { None } } } }
    };
}

impl_357!()