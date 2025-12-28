macro_rules! deps {
    () => {
        Error!();
        Parser!();
        MapRes!();
        ErrorKind!();
        PResult!();
        Err!();
        Emit!();
        FromExternalError!();
        OutputMode!();
        OutputM!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < I , O2 , E2 , F , G > Parser < I > for MapRes < F , G > where I : Clone , < F as Parser < I > > :: Error : FromExternalError < I , E2 > , F : Parser < I > , G : FnMut (< F as Parser < I > > :: Output) -> Result < O2 , E2 > , { type Output = O2 ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (input , o1) = self . f . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (i . clone ()) ? ; match (self . g) (o1) { Ok (o2) => Ok ((input , OM :: Output :: bind (| | o2))) , Err (e) => Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_external_error (i , ErrorKind :: MapRes , e) }))) , } } }
    };
}

impl_180!()