macro_rules! deps {
    () => {
        OutputMode!();
        PResult!();
        OutputM!();
        Parser!();
        ParseError!();
        Preceded!();
        Error!();
        Check!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < I , E : ParseError < I > , F : Parser < I , Error = E > , G : Parser < I , Error = E > > Parser < I > for Preceded < F , G > { type Output = < G as Parser < I > > :: Output ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (i , _) = self . f . process :: < OutputM < Check , OM :: Error , OM :: Incomplete > > (i) ? ; let (i , o2) = self . g . process :: < OM > (i) ? ; Ok ((i , o2)) } }
    };
}

impl_288!();