macro_rules! deps {
    () => {
        Err!();
        OutputMode!();
        PResult!();
        Error!();
        Parser!();
        Peek!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Peek < F > where I : Clone , F : Parser < I > , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OM > (input) { Ok ((_ , o)) => Ok ((i , o)) , Err (e) => Err (e) , } } }
    };
}

impl_105!()