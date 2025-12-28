macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl PartialEq < SyntaxText > for str { fn eq (& self , rhs : & SyntaxText) -> bool { rhs == self } }
    };
}

impl_91!();