macro_rules! deps {
    () => {
        Utf8PrefixComponent!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl fmt :: Display for Utf8PrefixComponent < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_69!()