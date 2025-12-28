macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl PartialEq < & '_ str > for SyntaxText { fn eq (& self , rhs : & & str) -> bool { self == * rhs } }
    };
}

impl_92!();