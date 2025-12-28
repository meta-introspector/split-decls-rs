macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Clone for SyntaxNode { # [inline] fn clone (& self) -> Self { self . data () . inc_rc () ; SyntaxNode { ptr : self . ptr } } }
    };
}

impl_14!();