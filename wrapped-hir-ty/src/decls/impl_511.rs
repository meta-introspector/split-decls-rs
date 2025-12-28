macro_rules! deps {
    () => {
        HirDisplayError!();
        HirDisplay!();
        HirFormatter!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Term < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { Term :: Ty (it) => it . hir_fmt (f) , Term :: Const (it) => it . hir_fmt (f) , } } }
    };
}

impl_511!();