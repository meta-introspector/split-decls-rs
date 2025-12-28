macro_rules! deps {
    () => {
        TupleField!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for TupleField { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write ! (f , "pub {}: " , self . name () . display (f . db , f . edition ())) ? ; self . ty (f . db) . hir_fmt (f) } }
    };
}

impl_203!()