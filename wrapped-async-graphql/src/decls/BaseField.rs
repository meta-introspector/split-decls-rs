macro_rules! deps {
    () => {
        TypeRef!();
        InputValue!();
    };
}

macro_rules! BaseField {
    () => {
        deps!();
        pub (crate) trait BaseField { fn ty (& self) -> & TypeRef ; fn argument (& self , name : & str) -> Option < & InputValue > ; }
    };
}

BaseField!();