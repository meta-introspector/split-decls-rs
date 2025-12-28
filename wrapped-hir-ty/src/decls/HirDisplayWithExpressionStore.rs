macro_rules! deps {
    () => {
        HirDisplayError!();
        HirFormatter!();
    };
}

macro_rules! HirDisplayWithExpressionStore {
    () => {
        deps!();
        pub trait HirDisplayWithExpressionStore < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > ; }
    };
}

HirDisplayWithExpressionStore!()