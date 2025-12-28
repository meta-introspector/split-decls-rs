macro_rules! deps {
    () => {
        TraitRef!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for TraitRef < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { self . trait_ref . hir_fmt (f) } }
    };
}

impl_219!();