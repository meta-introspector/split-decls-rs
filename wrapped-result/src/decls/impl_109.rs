macro_rules! deps {
    () => {
        Result!();
        NTSTATUS!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl core :: fmt :: Display for NTSTATUS { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_fmt (format_args ! ("{:#010X}" , self . 0)) } }
    };
}

impl_109!()