macro_rules! deps {
    () => {
        RPC_STATUS!();
        Result!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl core :: fmt :: Debug for RPC_STATUS { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_fmt (format_args ! ("RPC_STATUS({self})")) } }
    };
}

impl_117!()