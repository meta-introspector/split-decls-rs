macro_rules! deps {
    () => {
        HirFormatter!();
        HirDisplayWithExpressionStore!();
        HirDisplayError!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl < 'db > HirDisplayWithExpressionStore < 'db > for hir_def :: expr_store :: path :: GenericArg { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { match self { hir_def :: expr_store :: path :: GenericArg :: Type (ty) => ty . hir_fmt (f , store) , hir_def :: expr_store :: path :: GenericArg :: Const (_c) => { write ! (f , "<expr>") } hir_def :: expr_store :: path :: GenericArg :: Lifetime (lifetime) => lifetime . hir_fmt (f , store) , } } }
    };
}

impl_531!();