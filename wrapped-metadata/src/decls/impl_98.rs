macro_rules! deps {
    () => {
        Type!();
        InterfaceImpl!();
        TypeDef!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'a > InterfaceImpl < 'a > { pub fn class (& self) -> TypeDef < 'a > { self . row (0) } pub fn interface (& self , generics : & [Type]) -> Type { self . decode :: < TypeDefOrRef > (1) . ty (generics) } }
    };
}

impl_98!()