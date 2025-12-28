macro_rules! deps {
    () => {
        Result!();
        HRESULT!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl core :: fmt :: Debug for HRESULT { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_fmt (format_args ! ("HRESULT({self})")) } }
    };
}

impl_86!();