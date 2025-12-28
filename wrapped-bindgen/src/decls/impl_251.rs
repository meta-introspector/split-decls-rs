macro_rules! deps {
    () => {
        Dependencies!();
        CppDelegate!();
        TypeMap!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl Dependencies for CppDelegate { fn combine (& self , dependencies : & mut TypeMap) { self . method () . signature (self . def . namespace () , & []) . combine (dependencies) ; } }
    };
}

impl_251!()