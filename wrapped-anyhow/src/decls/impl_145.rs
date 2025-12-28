macro_rules! deps {
    () => {
        BoxedError!();
        Result!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Debug for BoxedError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . 0 , f) } }
    };
}

impl_145!()