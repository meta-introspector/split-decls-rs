macro_rules! deps {
    () => {
        PResult!();
        Input!();
        Choice!();
        Error!();
        OutputMode!();
        ParseError!();
        Parser!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < Input , Output , Error : ParseError < Input > , A : Parser < Input , Output = Output , Error = Error > > Parser < Input > for Choice < (A ,) > { type Output = Output ; type Error = Error ; # [inline] fn process < OM : crate :: OutputMode > (& mut self , input : Input ,) -> crate :: PResult < OM , Input , Self :: Output , Self :: Error > { self . parser . 0 . process :: < OM > (input) } }
    };
}

impl_58!();