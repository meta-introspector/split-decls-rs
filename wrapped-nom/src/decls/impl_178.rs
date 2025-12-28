macro_rules! deps {
    () => {
        ParseError!();
        OutputMode!();
        Parser!();
        Map!();
        Err!();
        Error!();
        PResult!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < I , O2 , E : ParseError < I > , F : Parser < I , Error = E > , G : FnMut (< F as Parser < I > > :: Output) -> O2 > Parser < I > for Map < F , G > { type Output = O2 ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self . f . process :: < OM > (i) { Err (e) => Err (e) , Ok ((i , o)) => Ok ((i , OM :: Output :: map (o , | o | (self . g) (o)))) , } } }
    };
}

impl_178!();