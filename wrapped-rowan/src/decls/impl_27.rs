macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Debug for SyntaxNode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SyntaxNode") . field ("kind" , & self . kind ()) . field ("text_range" , & self . text_range ()) . finish () } }
    };
}

impl_27!()