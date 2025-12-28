macro_rules! deps {
    () => {
        PResult!();
        OutputMode!();
        AsChar!();
        Input!();
        ErrorKind!();
        Error!();
        ParseError!();
        Digit1!();
        Parser!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl < I : Input , E : ParseError < I > > Parser < I > for Digit1 < E > where < I as Input > :: Item : AsChar , { type Output = I ; type Error = E ; # [inline] fn process < OM : crate :: OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { input . split_at_position_mode1 :: < OM , _ , _ > (| item | ! item . is_dec_digit () , ErrorKind :: Digit) } }
    };
}

impl_581!();