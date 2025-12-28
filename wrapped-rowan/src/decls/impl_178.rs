macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < N : AstNode > PartialEq for AstPtr < N > { fn eq (& self , other : & AstPtr < N >) -> bool { self . raw == other . raw } }
    };
}

impl_178!();