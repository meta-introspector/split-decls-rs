macro_rules! deps {
    () => {
        OutputM!();
        Error!();
        Parser!();
        Err!();
        OutputMode!();
        MapOpt!();
        PResult!();
        Emit!();
        ErrorKind!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < I , O2 , F , G > Parser < I > for MapOpt < F , G > where I : Clone , F : Parser < I > , G : FnMut (< F as Parser < I > > :: Output) -> Option < O2 > , { type Output = O2 ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (input , o1) = self . f . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (i . clone ()) ? ; match (self . g) (o1) { Some (o2) => Ok ((input , OM :: Output :: bind (| | o2))) , None => Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: MapOpt) }))) , } } }
    };
}

impl_182!()