macro_rules! deps {
    () => {
        ErrorKind!();
        Err!();
        Parser!();
        Not!();
        OutputMode!();
        PResult!();
        Error!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Not < F > where I : Clone , F : Parser < I > , { type Output = () ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OM > (input) { Ok (_) => Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: Not) }))) , Err (Err :: Error (_)) => Ok ((i , OM :: Output :: bind (| | ()))) , Err (e) => Err (e) , } } }
    };
}

impl_119!();