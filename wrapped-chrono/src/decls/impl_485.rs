macro_rules! deps {
    () => {
        Error!();
        NaiveTimeVisitor!();
        NaiveTime!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl de :: Visitor < '_ > for NaiveTimeVisitor { type Value = NaiveTime ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a formatted time string") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { value . parse () . map_err (E :: custom) } }
    };
}

impl_485!();