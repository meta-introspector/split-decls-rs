macro_rules! deps {
    () => {
        WIN32_ERROR!();
        Result!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl core :: fmt :: Display for WIN32_ERROR { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_fmt (format_args ! ("{}" , self . 0)) } }
    };
}

impl_102!()