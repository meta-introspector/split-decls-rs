macro_rules! deps {
    () => {
        Parser!();
        Error!();
        AsChar!();
        OutputMode!();
        Input!();
        PResult!();
        MultiSpace0!();
        ParseError!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > > Parser < I > for MultiSpace0 < Error > where I : Input , < I as Input > :: Item : AsChar , { type Output = I ; type Error = Error ; fn process < OM : crate :: OutputMode > (& mut self , i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { i . split_at_position_mode :: < OM , _ , _ > (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t' || c == '\r' || c == '\n') }) } }
    };
}

impl_584!()