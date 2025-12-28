macro_rules! deps {
    () => {
        Adt!();
        Union!();
        Enum!();
        Struct!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Adt { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { Adt :: Struct (it) => it . hir_fmt (f) , Adt :: Union (it) => it . hir_fmt (f) , Adt :: Enum (it) => it . hir_fmt (f) , } } }
    };
}

impl_196!()