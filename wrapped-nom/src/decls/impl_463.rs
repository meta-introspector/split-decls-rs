macro_rules! deps {
    () => {
        Parser!();
        Error!();
        PResult!();
        Input!();
        ParseError!();
        SplitPosition1!();
        OutputMode!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > , F > Parser < I > for SplitPosition1 < F , Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { type Output = I ; type Error = Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { i . split_at_position_mode1 :: < OM , _ , _ > (| c | (self . predicate) (c) , self . e) } }
    };
}

impl_463!();