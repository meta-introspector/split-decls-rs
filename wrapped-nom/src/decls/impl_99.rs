macro_rules! deps {
    () => {
        OutputM!();
        Check!();
        PResult!();
        Opt!();
        Error!();
        OutputMode!();
        Err!();
        Parser!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < I , F : Parser < I > > Parser < I > for Opt < F > where I : Clone , { type Output = Option < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OutputM < OM :: Output , Check , OM :: Incomplete > > (input) { Ok ((i , o)) => Ok ((i , OM :: Output :: map (o , Some))) , Err (Err :: Error (_)) => Ok ((i , OM :: Output :: bind (| | None))) , Err (Err :: Failure (e)) => Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => Err (Err :: Incomplete (i)) , } } }
    };
}

impl_99!()