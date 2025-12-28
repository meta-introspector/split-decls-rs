macro_rules! deps {
    () => {
        Demangle!();
        SizeLimitExhausted!();
        SizeLimitedFmtAdapter!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for Demangle < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . style { None => f . write_str (self . original) ? , Some (ref d) => { let alternate = f . alternate () ; let mut size_limited_fmt = SizeLimitedFmtAdapter { remaining : Ok (MAX_SIZE) , inner : & mut * f , } ; let fmt_result = if alternate { write ! (size_limited_fmt , "{:#}" , d) } else { write ! (size_limited_fmt , "{}" , d) } ; let size_limit_result = size_limited_fmt . remaining . map (| _ | ()) ; match (fmt_result , size_limit_result) { (Err (_) , Err (SizeLimitExhausted)) => f . write_str ("{size limit reached}") ? , _ => { fmt_result ? ; size_limit_result . expect ("`fmt::Error` from `SizeLimitedFmtAdapter` was discarded") ; } } } } f . write_str (self . suffix) } }
    };
}

impl_47!();