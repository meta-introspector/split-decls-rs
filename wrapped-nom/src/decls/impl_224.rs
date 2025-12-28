macro_rules! deps {
    () => {
        Err!();
        Many0!();
        Error!();
        Check!();
        Input!();
        Parser!();
        OutputMode!();
        PResult!();
        OutputM!();
        ErrorKind!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I , F > Parser < I > for Many0 < F > where I : Clone + Input , F : Parser < I > , { type Output = crate :: lib :: std :: vec :: Vec < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut acc = OM :: Output :: bind (| | crate :: lib :: std :: vec :: Vec :: with_capacity (4)) ; loop { let len = i . input_len () ; match self . parser . process :: < OutputM < OM :: Output , Check , OM :: Incomplete > > (i . clone ()) { Err (Err :: Error (_)) => return Ok ((i , acc)) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (e)) => return Err (Err :: Incomplete (e)) , Ok ((i1 , o)) => { if i1 . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: Many0) }))) ; } i = i1 ; acc = OM :: Output :: combine (acc , o , | mut acc , o | { acc . push (o) ; acc }) } } } } }
    };
}

impl_224!()