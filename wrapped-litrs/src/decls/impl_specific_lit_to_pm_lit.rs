macro_rules! deps {
    () => {
        Literal!();
        Buffer!();
    };
}

macro_rules! impl_specific_lit_to_pm_lit {
    () => {
        deps!();
        macro_rules ! impl_specific_lit_to_pm_lit { ([$ ($ prefix : tt) *] => $ ty : ident , $ variant : ident , $ kind : ident) => { impl < B : crate :: Buffer > From < crate ::$ ty < B >> for $ ($ prefix) * Literal { fn from (l : crate ::$ ty < B >) -> Self { l . raw_input () . parse () . unwrap_or_else (| e | { panic ! ("failed to parse `{}` as `{}`: {}" , l . raw_input () , std :: any :: type_name ::< Self > () , e ,) }) } } } ; }
    };
}

impl_specific_lit_to_pm_lit!();