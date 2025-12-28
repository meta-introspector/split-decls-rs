macro_rules! deps {
    () => {
        ParseError!();
        PResult!();
        Error!();
        OutputMode!();
        Parser!();
        Terminated!();
        OutputM!();
        Check!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < I , E : ParseError < I > , F : Parser < I , Error = E > , G : Parser < I , Error = E > > Parser < I > for Terminated < F , G > { type Output = < F as Parser < I > > :: Output ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (i , o1) = self . f . process :: < OM > (i) ? ; let (i , _) = self . g . process :: < OutputM < Check , OM :: Error , OM :: Incomplete > > (i) ? ; Ok ((i , o1)) } }
    };
}

impl_291!();