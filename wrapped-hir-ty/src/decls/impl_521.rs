macro_rules! deps {
    () => {
        HirDisplayWithExpressionStore!();
        HirDisplayError!();
        HirFormatter!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl < 'db , T : ? Sized + HirDisplayWithExpressionStore < 'db > > HirDisplayWithExpressionStore < 'db > for & '_ T { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { T :: hir_fmt (& * * self , f , store) } }
    };
}

impl_521!()