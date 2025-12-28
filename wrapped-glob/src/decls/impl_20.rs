macro_rules! deps {
    () => {
        Pattern!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [doc = " Show the original glob pattern."] impl fmt :: Display for Pattern { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . original . fmt (f) } }
    };
}

impl_20!();