macro_rules! deps {
    () => {
        LifetimeSyntaxCategories!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < T > LifetimeSyntaxCategories < Vec < T > > { pub fn len (& self) -> LifetimeSyntaxCategories < usize > { LifetimeSyntaxCategories { hidden : self . hidden . len () , elided : self . elided . len () , named : self . named . len () , } } pub fn iter_unnamed (& self) -> impl Iterator < Item = & T > { let Self { hidden , elided , named : _ } = self ; [hidden . iter () , elided . iter ()] . into_iter () . flatten () } }
    };
}

impl_360!();