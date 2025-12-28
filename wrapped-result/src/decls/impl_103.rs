macro_rules! deps {
    () => {
        WIN32_ERROR!();
        Result!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl core :: fmt :: Debug for WIN32_ERROR { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_fmt (format_args ! ("WIN32_ERROR({self})")) } }
    };
}

impl_103!()