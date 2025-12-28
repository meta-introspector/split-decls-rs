macro_rules! deps {
    () => {
        Parser!();
        Success!();
        Error!();
        OutputMode!();
        ParseError!();
        PResult!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < I , O , E > Parser < I > for Success < O , E > where O : Clone , E : ParseError < I > , { type Output = O ; type Error = E ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { Ok ((input , OM :: Output :: bind (| | self . val . clone ()))) } }
    };
}

impl_137!();