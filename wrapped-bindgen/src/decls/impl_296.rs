macro_rules! deps {
    () => {
        Delegate!();
        TypeMap!();
        Dependencies!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl Dependencies for Delegate { fn combine (& self , dependencies : & mut TypeMap) { dependencies . combine (& self . method () . dependencies) ; for ty in & self . generics { ty . combine (dependencies) ; } } }
    };
}

impl_296!();