macro_rules! deps {
    () => {
        Crate!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Crate { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . display_name (f . db) { Some (name) => write ! (f , "extern crate {name}") , None => f . write_str ("extern crate {unknown}") , } } }
    };
}

impl_224!();