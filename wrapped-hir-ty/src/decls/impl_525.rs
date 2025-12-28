macro_rules! deps {
    () => {
        HirDisplayWithExpressionStore!();
        HirDisplay!();
        HirDisplayError!();
        HirFormatter!();
        ExpressionStoreAdapter!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl < 'db , T : HirDisplayWithExpressionStore < 'db > > HirDisplay < 'db > for ExpressionStoreAdapter < '_ , T > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { T :: hir_fmt (& self . 0 , f , self . 1) } }
    };
}

impl_525!()