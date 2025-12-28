macro_rules! deps {
    () => {
        BaseField!();
        InterfaceField!();
        InputValue!();
        TypeRef!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl BaseField for InterfaceField { # [inline] fn ty (& self) -> & TypeRef { & self . ty } # [inline] fn argument (& self , name : & str) -> Option < & InputValue > { self . arguments . get (name) } }
    };
}

impl_407!()