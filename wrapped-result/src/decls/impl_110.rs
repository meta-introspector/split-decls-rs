macro_rules! deps {
    () => {
        NTSTATUS!();
        Result!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl core :: fmt :: Debug for NTSTATUS { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_fmt (format_args ! ("NTSTATUS({self})")) } }
    };
}

impl_110!();