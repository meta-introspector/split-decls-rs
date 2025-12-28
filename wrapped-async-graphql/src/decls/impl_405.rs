macro_rules! deps {
    () => {
        BaseField!();
        InputValue!();
        TypeRef!();
        Field!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl BaseField for Field { # [inline] fn ty (& self) -> & TypeRef { & self . ty } # [inline] fn argument (& self , name : & str) -> Option < & InputValue > { self . arguments . get (name) } }
    };
}

impl_405!();