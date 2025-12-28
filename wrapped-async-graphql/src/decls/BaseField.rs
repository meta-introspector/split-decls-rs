macro_rules! deps {
    () => {
        InputValue!();
        TypeRef!();
    };
}

macro_rules! BaseField {
    () => {
        deps!();
        pub (crate) trait BaseField { fn ty (& self) -> & TypeRef ; fn argument (& self , name : & str) -> Option < & InputValue > ; }
    };
}

BaseField!()