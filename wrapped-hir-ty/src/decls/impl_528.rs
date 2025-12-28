macro_rules! deps {
    () => {
        HirDisplayError!();
        HirFormatter!();
        HirDisplayWithExpressionStore!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl < 'db > HirDisplayWithExpressionStore < 'db > for ConstRef { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , _store : & ExpressionStore ,) -> Result < () , HirDisplayError > { write ! (f , "{{const}}") ? ; Ok (()) } }
    };
}

impl_528!();