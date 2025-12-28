macro_rules! deps {
    () => {
        HirDisplay!();
        HirFormatter!();
        HirDisplayError!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for GenericArg < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { GenericArg :: Ty (ty) => ty . hir_fmt (f) , GenericArg :: Lifetime (lt) => lt . hir_fmt (f) , GenericArg :: Const (c) => c . hir_fmt (f) , } } }
    };
}

impl_499!()