macro_rules! deps {
    () => {
        WindowsError!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl core :: fmt :: Debug for WindowsError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_75!();