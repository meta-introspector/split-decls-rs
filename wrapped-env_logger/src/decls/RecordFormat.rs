macro_rules! deps {
    () => {
        Formatter!();
    };
}

macro_rules! RecordFormat {
    () => {
        deps!();
        pub (crate) trait RecordFormat { fn format (& self , formatter : & mut Formatter , record : & Record < '_ >) -> io :: Result < () > ; }
    };
}

RecordFormat!();