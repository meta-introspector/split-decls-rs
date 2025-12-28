macro_rules! deps {
    () => {
        Parser!();
        Many1Count!();
        Check!();
        Error!();
        Input!();
        PResult!();
        OutputM!();
        Err!();
        ErrorKind!();
        OutputMode!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Many1Count < F > where I : Clone + Input , F : Parser < I > , { type Output = usize ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut count = 0 ; match self . parser . process :: < OutputM < Check , Check , OM :: Incomplete > > (input . clone ()) { Err (Err :: Error (_)) => Err (Err :: Error (OM :: Error :: bind (move | | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Many1Count) }))) , Err (Err :: Failure (e)) => Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => Err (Err :: Incomplete (i)) , Ok ((mut input , _)) => { count += 1 ; loop { let input_ = input . clone () ; let len = input . input_len () ; match self . parser . process :: < OutputM < Check , Check , OM :: Incomplete > > (input_) { Ok ((i , _)) => { if i . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Many1Count) }))) ; } input = i ; count += 1 ; } Err (Err :: Error (_)) => return Ok ((input , OM :: Output :: bind (| | count))) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => return Err (Err :: Incomplete (i)) , } } } } } }
    };
}

impl_245!();