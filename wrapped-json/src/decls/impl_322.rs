macro_rules! deps {
    () => {
        Result!();
        Formatter!();
        Value!();
        Type!();
        Number!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < 'a > Display for Type < 'a > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match * self . 0 { Value :: Null => formatter . write_str ("null") , Value :: Bool (_) => formatter . write_str ("boolean") , Value :: Number (_) => formatter . write_str ("number") , Value :: String (_) => formatter . write_str ("string") , Value :: Array (_) => formatter . write_str ("array") , Value :: Object (_) => formatter . write_str ("object") , } } }
    };
}

impl_322!();