macro_rules! deps {
    () => {
        TypeOrConstParam!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for TypeOrConstParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . split (f . db) { either :: Either :: Left (it) => it . hir_fmt (f) , either :: Either :: Right (it) => it . hir_fmt (f) , } } }
    };
}

impl_209!()