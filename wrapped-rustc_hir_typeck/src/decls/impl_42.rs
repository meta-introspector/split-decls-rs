macro_rules! deps {
    () => {
        AsCoercionSite!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'a , T > AsCoercionSite for & 'a T where T : AsCoercionSite , { fn as_coercion_site (& self) -> & hir :: Expr < '_ > { (* * self) . as_coercion_site () } }
    };
}

impl_42!();