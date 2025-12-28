macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Display for Error < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{} at byte {}: {:?}" , self . error , self . pos , format_args ! ("{}(HERE-->){}" , & self . input [.. self . pos] , & self . input [self . pos ..]) ,) } }
    };
}

impl_5!();