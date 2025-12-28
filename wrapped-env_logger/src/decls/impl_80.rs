macro_rules! deps {
    () => {
        Formatter!();
        RecordFormat!();
        ConfigurableFormat!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl RecordFormat for ConfigurableFormat { fn format (& self , formatter : & mut Formatter , record : & Record < '_ >) -> io :: Result < () > { self . format (formatter , record) } }
    };
}

impl_80!();