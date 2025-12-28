macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl PartialEq < SyntaxText > for & '_ str { fn eq (& self , rhs : & SyntaxText) -> bool { rhs == self } }
    };
}

impl_93!()