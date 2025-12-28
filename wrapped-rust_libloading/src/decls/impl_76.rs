macro_rules! deps {
    () => {
        Error!();
        WindowsError!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl core :: fmt :: Display for WindowsError { # [cfg (feature = "std")] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let error = std :: io :: Error :: from_raw_os_error (self . 0) ; core :: fmt :: Display :: fmt (& error , f) } # [cfg (not (feature = "std"))] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_fmt (format_args ! ("OS error {}" , self . 0)) } }
    };
}

impl_76!()