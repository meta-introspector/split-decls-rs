macro_rules! deps {
    () => {
        PResult!();
        ManyMN!();
        Input!();
        Error!();
        Err!();
        ErrorKind!();
        Parser!();
        OutputMode!();
        FoldManyMN!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < I , F , G , Init , R > Parser < I > for FoldManyMN < F , G , Init , R > where I : Clone + Input , F : Parser < I > , G : FnMut (R , < F as Parser < I > > :: Output) -> R , Init : FnMut () -> R , { type Output = R ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { if self . min > self . max { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: ManyMN) }))) ; } let mut res = OM :: Output :: bind (| | (self . init) ()) ; for count in 0 .. self . max { let len = input . input_len () ; match self . parser . process :: < OM > (input . clone ()) { Ok ((tail , value)) => { if tail . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (tail , ErrorKind :: ManyMN) }))) ; } res = OM :: Output :: combine (res , value , | res , o | (self . g) (res , o)) ; input = tail ; } Err (Err :: Error (err)) => { if count < self . min { return Err (Err :: Error (OM :: Error :: map (err , | err | { < F as Parser < I > > :: Error :: append (input , ErrorKind :: ManyMN , err) }))) ; } else { break ; } } Err (e) => return Err (e) , } } Ok ((input , res)) } }
    };
}

impl_260!();