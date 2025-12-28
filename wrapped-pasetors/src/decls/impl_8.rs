macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_fmt (format_args ! ("{self:?}")) } }
    };
}

impl_8!()