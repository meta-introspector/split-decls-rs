macro_rules! deps {
    () => {
        ParseWeekdayError!();
    };
}

macro_rules! impl_704 {
    () => {
        deps!();
        impl fmt :: Display for ParseWeekdayError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_fmt (format_args ! ("{self:?}")) } }
    };
}

impl_704!()