macro_rules! deps {
    () => {
        NaiveDateTimeVisitor!();
        Error!();
        NaiveDateTime!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl de :: Visitor < '_ > for NaiveDateTimeVisitor { type Value = NaiveDateTime ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a formatted date and time string") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { value . parse () . map_err (E :: custom) } }
    };
}

impl_382!()