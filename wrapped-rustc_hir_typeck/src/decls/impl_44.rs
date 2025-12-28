macro_rules! deps {
    () => {
        AsCoercionSite!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl AsCoercionSite for hir :: Arm < '_ > { fn as_coercion_site (& self) -> & hir :: Expr < '_ > { self . body } }
    };
}

impl_44!()