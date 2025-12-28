macro_rules! deps {
    () => {
        Parser!();
        Err!();
        OutputMode!();
        IResult!();
        Error!();
        ParseError!();
        PResult!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < I , O , E : ParseError < I > , F > Parser < I > for F where F : FnMut (I) -> IResult < I , O , E > , { type Output = O ; type Error = E ; fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (i , o) = self (i) . map_err (| e | match e { Err :: Incomplete (i) => Err :: Incomplete (i) , Err :: Error (e) => Err :: Error (OM :: Error :: bind (| | e)) , Err :: Failure (e) => Err :: Failure (e) , }) ? ; Ok ((i , OM :: Output :: bind (| | o))) } }
    };
}

impl_173!();