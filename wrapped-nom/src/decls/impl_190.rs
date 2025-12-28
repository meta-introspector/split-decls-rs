macro_rules! deps {
    () => {
        ParseError!();
        Error!();
        OutputMode!();
        Or!();
        PResult!();
        Err!();
        Parser!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < I : Clone , O , E : ParseError < I > , F : Parser < I , Output = O , Error = E > , G : Parser < I , Output = O , Error = E > , > Parser < I > for Or < F , G > { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self . f . process :: < OM > (i . clone ()) { Err (Err :: Error (e1)) => match self . g . process :: < OM > (i) { Err (Err :: Error (e2)) => Err (Err :: Error (OM :: Error :: combine (e1 , e2 , | e1 , e2 | e1 . or (e2)))) , res => res , } , res => res , } } }
    };
}

impl_190!()