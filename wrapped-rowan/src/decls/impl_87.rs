macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl fmt :: Debug for SyntaxText { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . to_string () , f) } }
    };
}

impl_87!()