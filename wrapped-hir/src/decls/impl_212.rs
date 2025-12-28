macro_rules! deps {
    () => {
        ConstParam!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for ConstParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write ! (f , "const {}: " , self . name (f . db) . display (f . db , f . edition ())) ? ; self . ty (f . db) . hir_fmt (f) } }
    };
}

impl_212!();