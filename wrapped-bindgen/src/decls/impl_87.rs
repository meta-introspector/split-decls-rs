macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl InterfaceImpl { pub fn ty (& self , generics : & [Type]) -> Type { Type :: from_ref (self . decode (1) , None , generics) } }
    };
}

impl_87!()