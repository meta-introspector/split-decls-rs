macro_rules! deps {
    () => {
        Type!();
        TypeSpec!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl TypeSpec < '_ > { pub fn ty (& self , generics : & [Type]) -> Type { self . blob (0) . read_type_code (generics) } }
    };
}

impl_124!();