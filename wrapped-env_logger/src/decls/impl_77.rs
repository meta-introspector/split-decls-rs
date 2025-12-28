macro_rules! deps {
    () => {
        ConfigurableFormat!();
        ConfigurableFormatWriter!();
        Formatter!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl ConfigurableFormat { # [doc = " Format the [`Record`] as configured for outputting"] pub fn format (& self , formatter : & mut Formatter , record : & Record < '_ >) -> io :: Result < () > { let fmt = ConfigurableFormatWriter { format : self , buf : formatter , written_header_value : false , } ; fmt . write (record) } }
    };
}

impl_77!();