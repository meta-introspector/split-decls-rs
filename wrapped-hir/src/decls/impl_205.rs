macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Type < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { self . ty . hir_fmt (f) } }
    };
}

impl_205!();