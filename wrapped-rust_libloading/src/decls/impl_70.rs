macro_rules! deps {
    () => {
        DlError!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl core :: fmt :: Debug for DlError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_70!()