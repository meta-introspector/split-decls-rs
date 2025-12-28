macro_rules! deps {
    () => {
        Interface!();
        TypeMap!();
        TypeName!();
        Type!();
        Dependencies!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl Dependencies for Interface { fn combine (& self , dependencies : & mut TypeMap) { Type :: Object . combine (dependencies) ; for interface in self . required_interfaces () { Type :: Interface (interface) . combine (dependencies) ; } for ty in & self . generics { ty . combine (dependencies) ; } let is_iterable = self . type_name () == TypeName :: IIterable ; for method in self . def . methods () { for ty in method . signature (self . def . namespace () , & self . generics) . types () { if is_iterable || ty . is_core () { ty . combine (dependencies) ; } } } } }
    };
}

impl_308!()