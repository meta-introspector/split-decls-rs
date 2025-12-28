macro_rules! deps {
    () => {
        ParseError!();
        Parser!();
        OutputMode!();
        OutputM!();
        Err!();
        Input!();
        Check!();
        ErrorKind!();
        Error!();
        PResult!();
        SeparatedList1!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I , E : ParseError < I > , F , G > Parser < I > for SeparatedList1 < F , G > where I : Clone + Input , F : Parser < I , Error = E > , G : Parser < I , Error = E > , { type Output = Vec < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut res = OM :: Output :: bind (crate :: lib :: std :: vec :: Vec :: new) ; match self . parser . process :: < OM > (i . clone ()) { Err (e) => return Err (e) , Ok ((i1 , o)) => { res = OM :: Output :: combine (res , o , | mut res , o | { res . push (o) ; res }) ; i = i1 ; } } loop { let len = i . input_len () ; match self . separator . process :: < OutputM < Check , Check , OM :: Incomplete > > (i . clone ()) { Err (Err :: Error (_)) => return Ok ((i , res)) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (e)) => return Err (Err :: Incomplete (e)) , Ok ((i1 , _)) => { match self . parser . process :: < OutputM < OM :: Output , Check , OM :: Incomplete > > (i1 . clone ()) { Err (Err :: Error (_)) => return Ok ((i , res)) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (e)) => return Err (Err :: Incomplete (e)) , Ok ((i2 , o)) => { if i2 . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: SeparatedList) }))) ; } res = OM :: Output :: combine (res , o , | mut res , o | { res . push (o) ; res }) ; i = i2 ; } } } } } } }
    };
}

impl_236!()