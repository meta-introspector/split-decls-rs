macro_rules! deps {
    () => {
        MonikerIdentifier!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl fmt :: Display for MonikerIdentifier { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& self . crate_name) ? ; f . write_fmt (format_args ! ("::{}" , self . description . iter () . map (| x | & x . name) . join ("::"))) } }
    };
}

impl_314!()