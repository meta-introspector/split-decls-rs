macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        # [cfg (feature = "signal")] impl fmt :: Display for Signal { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (self . as_ref ()) } }
    };
}

impl_119!();