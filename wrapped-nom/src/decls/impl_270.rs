macro_rules! deps {
    () => {
        NomRange!();
        ErrorKind!();
        OutputMode!();
        PResult!();
        Input!();
        Parser!();
        Many!();
        Error!();
        Err!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < I , F , R , Collection > Parser < I > for Many < F , R , Collection > where I : Clone + Input , F : Parser < I > , Collection : Extend < < F as Parser < I > > :: Output > + Default , R : NomRange < usize > , { type Output = Collection ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { if self . range . is_inverted () { return Err (Err :: Failure (< F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Many ,))) ; } let mut res = OM :: Output :: bind (Collection :: default) ; for count in self . range . bounded_iter () { let len = input . input_len () ; match self . parser . process :: < OM > (input . clone ()) { Ok ((tail , value)) => { if tail . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Many) }))) ; } res = OM :: Output :: combine (res , value , | mut res , value | { res . extend (Some (value)) ; res }) ; input = tail ; } Err (Err :: Error (e)) => { if ! self . range . contains (& count) { return Err (Err :: Error (OM :: Error :: map (e , | e | { < F as Parser < I > > :: Error :: append (input , ErrorKind :: Many , e) }))) ; } else { return Ok ((input , res)) ; } } Err (e) => { return Err (e) ; } } } Ok ((input , res)) } }
    };
}

impl_270!()