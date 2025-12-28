macro_rules! deps {
    () => {
        Emit!();
        Parser!();
        Err!();
        OutputM!();
        Error!();
        OutputMode!();
        Cut!();
        PResult!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Cut < F > where F : Parser < I > , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self . parser . process :: < OutputM < OM :: Output , Emit , OM :: Incomplete > > (input) { Err (Err :: Error (e)) => Err (Err :: Failure (e)) , Err (Err :: Failure (e)) => Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => Err (Err :: Incomplete (i)) , Ok ((i , o)) => Ok ((i , o)) , } } }
    };
}

impl_128!()