macro_rules! deps {
    () => {
        HirDisplay!();
        HirFormatter!();
        HirDisplayError!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        impl < 'db , T : HirDisplay < 'db > + Internable > HirDisplay < 'db > for Interned < T > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { HirDisplay :: hir_fmt (self . as_ref () , f) } }
    };
}

impl_497!();