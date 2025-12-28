macro_rules! deps {
    () => {
        OutputM!();
        Err!();
        ErrorKind!();
        Error!();
        PResult!();
        Verify!();
        Parser!();
        Emit!();
        OutputMode!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < I , F : Parser < I > , G , O2 > Parser < I > for Verify < F , G , O2 > where I : Clone , G : Fn (& O2) -> bool , < F as Parser < I > > :: Output : Borrow < O2 > , O2 : ? Sized , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (i , o) = self . first . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (input . clone ()) ? ; if (self . second) (o . borrow ()) { Ok ((i , OM :: Output :: bind (| | o))) } else { Err (Err :: Error (OM :: Error :: bind (move | | { let e : ErrorKind = ErrorKind :: Verify ; < F as Parser < I > > :: Error :: from_error_kind (input , e) }))) } } }
    };
}

impl_115!();