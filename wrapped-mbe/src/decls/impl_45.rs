macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ParseError :: UnexpectedToken (it) => f . write_str (it) , ParseError :: Expected (it) => f . write_str (it) , ParseError :: InvalidRepeat => f . write_str ("invalid repeat") , ParseError :: RepetitionEmptyTokenTree => f . write_str ("empty token tree in repetition") , } } }
    };
}

impl_45!()