macro_rules! deps {
    () => {
        UseTree!();
        Use!();
        ImportAlias!();
        ImportKind!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl Use { # [doc = " Expands the `UseTree` into individually imported `ModPath`s."] pub fn expand (& self , mut cb : impl FnMut (Idx < ast :: UseTree > , ModPath , ImportKind , Option < ImportAlias >) ,) { self . use_tree . expand_impl (None , & mut 0 , & mut cb) } }
    };
}

impl_192!();