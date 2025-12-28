macro_rules! deps {
    () => {
        RangeEnd!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl fmt :: Display for RangeEnd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { RangeEnd :: Included => "..=" , RangeEnd :: Excluded => ".." , }) } }
    };
}

impl_185!();