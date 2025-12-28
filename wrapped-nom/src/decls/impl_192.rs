macro_rules! deps {
    () => {
        Error!();
        Parser!();
        PResult!();
        Into!();
        ParseError!();
        OutputMode!();
        Err!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < I , O2 : From < < F as Parser < I > > :: Output > , E2 : crate :: error :: ParseError < I > + From < < F as Parser < I > > :: Error > , F : Parser < I > , > Parser < I > for Into < F , O2 , E2 > { type Output = O2 ; type Error = E2 ; fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self . f . process :: < OM > (i) { Ok ((i , o)) => Ok ((i , OM :: Output :: map (o , | o | o . into ()))) , Err (Err :: Error (e)) => Err (Err :: Error (OM :: Error :: map (e , | e | e . into ()))) , Err (Err :: Failure (e)) => Err (Err :: Failure (e . into ())) , Err (Err :: Incomplete (e)) => Err (Err :: Incomplete (e)) , } } }
    };
}

impl_192!()