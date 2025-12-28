macro_rules! deps {
    () => {
        BytesOrWideString!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > fmt :: Display for BytesOrWideString < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . to_str_lossy () . fmt (f) } }
    };
}

impl_33!();