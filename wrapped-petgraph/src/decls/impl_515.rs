macro_rules! deps {
    () => {
        Csr!();
        CsrError!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl fmt :: Display for CsrError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CsrError :: IndicesOutBounds (a , b) => { write ! (f , "Both node indices {a} and {b} is out of Csr bounds") } } } }
    };
}

impl_515!()