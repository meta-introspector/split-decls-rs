macro_rules! deps {
    () => {
        Dependencies!();
        CppInterface!();
        TypeMap!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl Dependencies for CppInterface { fn combine (& self , dependencies : & mut TypeMap) { let base_interfaces = self . base_interfaces () ; for interface in & base_interfaces { interface . combine (dependencies) ; } for method in self . def . methods () { for ty in method . signature (self . def . namespace () , & []) . types () { if ty . is_core () { ty . combine (dependencies) ; } } } } }
    };
}

impl_273!()