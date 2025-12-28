macro_rules! deps {
    () => {
        RPC_STATUS!();
        Result!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl core :: fmt :: Display for RPC_STATUS { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_fmt (format_args ! ("{:#010X}" , self . 0)) } }
    };
}

impl_116!();