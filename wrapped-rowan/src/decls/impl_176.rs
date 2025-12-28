macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < N : AstNode > fmt :: Debug for AstPtr < N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AstPtr") . field ("raw" , & self . raw) . finish () } }
    };
}

impl_176!()