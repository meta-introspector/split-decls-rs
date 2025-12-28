macro_rules! deps {
    () => {
        FromUtf16Error!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl fmt :: Display for FromUtf16Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Capacity (err) => write ! (f , "{err}") , Self :: DecodeUtf16 (err) => write ! (f , "invalid UTF-16: {err}") , } } }
    };
}

impl_221!()