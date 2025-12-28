macro_rules! deps {
    () => {
        FlatMap!();
        ParseError!();
        OutputMode!();
        PResult!();
        OutputM!();
        Emit!();
        Parser!();
        Error!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < I , E : ParseError < I > , F : Parser < I , Error = E > , G : FnMut (< F as Parser < I > > :: Output) -> H , H : Parser < I , Error = E > , > Parser < I > for FlatMap < F , G > { type Output = < H as Parser < I > > :: Output ; type Error = E ; fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (input , o1) = self . f . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (i) ? ; (self . g) (o1) . process :: < OM > (input) } }
    };
}

impl_184!();