macro_rules! deps {
    () => {
        RecordFormat!();
        Formatter!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < F > RecordFormat for F where F : Fn (& mut Formatter , & Record < '_ >) -> io :: Result < () > , { fn format (& self , formatter : & mut Formatter , record : & Record < '_ >) -> io :: Result < () > { (self) (formatter , record) } }
    };
}

impl_67!();