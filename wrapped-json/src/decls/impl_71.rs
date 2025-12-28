macro_rules! deps {
    () => {
        Result!();
        JsonUnexpected!();
        Float!();
        Formatter!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a > Display for JsonUnexpected < 'a > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { de :: Unexpected :: Unit => formatter . write_str ("null") , de :: Unexpected :: Float (value) => write ! (formatter , "floating point `{}`" , ryu :: Buffer :: new () . format (value) ,) , unexp => Display :: fmt (& unexp , formatter) , } } }
    };
}

impl_71!()