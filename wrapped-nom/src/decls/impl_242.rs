macro_rules! deps {
    () => {
        Parser!();
        Check!();
        Err!();
        ErrorKind!();
        Input!();
        OutputMode!();
        PResult!();
        OutputM!();
        Many0Count!();
        Error!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Many0Count < F > where I : Clone + Input , F : Parser < I > , { type Output = usize ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut count = 0 ; loop { let input_ = input . clone () ; let len = input . input_len () ; match self . parser . process :: < OutputM < Check , Check , OM :: Incomplete > > (input_) { Ok ((i , _)) => { if i . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Many0Count) }))) ; } input = i ; count += 1 ; } Err (Err :: Error (_)) => return Ok ((input , OM :: Output :: bind (| | count))) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => return Err (Err :: Incomplete (i)) , } } } }
    };
}

impl_242!();