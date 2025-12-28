macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl :: core :: fmt :: Debug for Error { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { match * self { Error :: InvalidLength (len) => write ! (f , "Invalid input length {len}") , Error :: InvalidChar => write ! (f , "Invalid character") , Error :: Overflow => write ! (f , "Overflow") , } } }
    };
}

impl_54!()