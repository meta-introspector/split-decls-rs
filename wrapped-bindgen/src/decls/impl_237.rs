macro_rules! deps {
    () => {
        Dependencies!();
        Type!();
        Class!();
        TypeMap!();
        Interface!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl Dependencies for Class { fn combine (& self , dependencies : & mut TypeMap) { for interface in self . required_interfaces () { Type :: Interface (interface) . combine (dependencies) ; } } }
    };
}

impl_237!()