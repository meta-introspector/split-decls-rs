macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Field { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_visibility (self . parent . module (f . db) . id , self . visibility (f . db) , f) ? ; write ! (f , "{}: " , self . name (f . db) . display (f . db , f . edition ())) ? ; self . ty (f . db) . hir_fmt (f) } }
    };
}

impl_202!()