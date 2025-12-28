macro_rules! deps {
    () => {
        TypeNs!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for TypeNs < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { self . ty . hir_fmt (f) } }
    };
}

impl_206!();