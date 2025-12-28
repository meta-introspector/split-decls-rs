macro_rules! deps {
    () => {
        CppConst!();
        TypeMap!();
        Dependencies!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl Dependencies for CppConst { fn combine (& self , dependencies : & mut TypeMap) { self . field . ty (None) . to_const_type () . combine (dependencies) ; } }
    };
}

impl_243!();