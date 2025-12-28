macro_rules! deps {
    () => {
        TypeParam!();
        ConstParam!();
        GenericParam!();
        LifetimeParam!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for GenericParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { GenericParam :: TypeParam (it) => it . hir_fmt (f) , GenericParam :: ConstParam (it) => it . hir_fmt (f) , GenericParam :: LifetimeParam (it) => it . hir_fmt (f) , } } }
    };
}

impl_208!();