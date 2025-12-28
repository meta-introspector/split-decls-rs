macro_rules! deps {
    () => {
        HirDisplayError!();
        HirFormatter!();
        HirDisplay!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl < 'db , T : HirDisplay < 'db > > HirDisplay < 'db > for & T { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { HirDisplay :: hir_fmt (* self , f) } }
    };
}

impl_496!()