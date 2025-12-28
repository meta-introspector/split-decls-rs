macro_rules! deps {
    () => {
        FixedOffset!();
        Error!();
        DateTimeVisitor!();
        DateTime!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl de :: Visitor < '_ > for DateTimeVisitor { type Value = DateTime < FixedOffset > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("an RFC 3339 formatted date and time string") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { value . parse () . map_err (E :: custom) } }
    };
}

impl_65!()