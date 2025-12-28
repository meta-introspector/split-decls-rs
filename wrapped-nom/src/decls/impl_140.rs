macro_rules! deps {
    () => {
        Parser!();
        Fail!();
        ErrorKind!();
        OutputMode!();
        Error!();
        Err!();
        PResult!();
        ParseError!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < I , O , E > Parser < I > for Fail < O , E > where E : ParseError < I > , { type Output = O ; type Error = E ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { Err (Err :: Error (OM :: Error :: bind (| | { E :: from_error_kind (input , ErrorKind :: Fail) }))) } }
    };
}

impl_140!();