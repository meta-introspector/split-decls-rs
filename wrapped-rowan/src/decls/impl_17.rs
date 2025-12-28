macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Clone for SyntaxToken { # [inline] fn clone (& self) -> Self { self . data () . inc_rc () ; SyntaxToken { ptr : self . ptr } } }
    };
}

impl_17!()