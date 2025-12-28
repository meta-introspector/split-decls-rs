macro_rules! deps {
    () => {
        AsCoercionSite!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl AsCoercionSite for hir :: Expr < '_ > { fn as_coercion_site (& self) -> & hir :: Expr < '_ > { self } }
    };
}

impl_41!()