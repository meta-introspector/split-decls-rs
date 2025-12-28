macro_rules! deps {
    () => {
        Parser!();
        ContextError!();
        OutputMode!();
        PResult!();
        Err!();
        Error!();
        Context!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Context < F > where I : Clone , F : Parser < I > , < F as Parser < I > > :: Error : ContextError < I > , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self . parser . process :: < OM > (input . clone ()) { Err (Err :: Error (e)) => Err (Err :: Error (OM :: Error :: map (e , | e | { < F as Parser < I > > :: Error :: add_context (input , self . context , e) }))) , Err (Err :: Failure (e)) => Err (Err :: Failure (< F as Parser < I > > :: Error :: add_context (input , self . context , e ,))) , x => x , } } }
    };
}

impl_32!()