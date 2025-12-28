macro_rules! deps {
    () => {
        LifetimeParam!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for LifetimeParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write ! (f , "{}" , self . name (f . db) . display (f . db , f . edition ())) } }
    };
}

impl_211!()