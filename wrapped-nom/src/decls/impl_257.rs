macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        Input!();
        Err!();
        OutputMode!();
        Parser!();
        PResult!();
        FoldMany1!();
        Many1!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < I , F , G , Init , R > Parser < I > for FoldMany1 < F , G , Init , R > where I : Clone + Input , F : Parser < I > , G : FnMut (R , < F as Parser < I > > :: Output) -> R , Init : FnMut () -> R , { type Output = R ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut res = OM :: Output :: bind (| | (self . init) ()) ; let input = i . clone () ; match self . parser . process :: < OM > (input) { Err (Err :: Error (_)) => Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: Many1) }))) , Err (e) => Err (e) , Ok ((i1 , o1)) => { res = OM :: Output :: combine (res , o1 , | res , o | (self . g) (res , o)) ; let mut input = i1 ; loop { let i_ = input . clone () ; let len = input . input_len () ; match self . parser . process :: < OM > (i_) { Ok ((i , o)) => { if i . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Many1) }))) ; } res = OM :: Output :: combine (res , o , | res , o | (self . g) (res , o)) ; input = i ; } Err (Err :: Error (_)) => { return Ok ((input , res)) ; } Err (e) => { return Err (e) ; } } } } } } }
    };
}

impl_257!()