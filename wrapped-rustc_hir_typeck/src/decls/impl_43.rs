macro_rules! deps {
    () => {
        AsCoercionSite!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl AsCoercionSite for ! { fn as_coercion_site (& self) -> & hir :: Expr < '_ > { * self } }
    };
}

impl_43!()