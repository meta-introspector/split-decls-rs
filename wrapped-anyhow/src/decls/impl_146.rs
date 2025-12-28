macro_rules! deps {
    () => {
        BoxedError!();
        Result!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Display for BoxedError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . 0 , f) } }
    };
}

impl_146!();