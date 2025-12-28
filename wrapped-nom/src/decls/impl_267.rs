macro_rules! deps {
    () => {
        OutputM!();
        Count!();
        LengthCount!();
        PResult!();
        Parser!();
        Err!();
        ParseError!();
        Error!();
        ToUsize!();
        Emit!();
        OutputMode!();
        ErrorKind!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I , F , G , E > Parser < I > for LengthCount < F , G , E > where I : Clone , F : Parser < I , Error = E > , G : Parser < I , Error = E > , < F as Parser < I > > :: Output : ToUsize , E : ParseError < I > , { type Output = Vec < < G as Parser < I > > :: Output > ; type Error = E ; fn process < OM : OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match self . length . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (input) { Err (e) => Err (e) , Ok ((i , count)) => { let count = count . to_usize () ; let mut input = i . clone () ; let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES / crate :: lib :: std :: mem :: size_of :: < < F as Parser < I > > :: Output > () . max (1) ; let mut res = OM :: Output :: bind (| | { crate :: lib :: std :: vec :: Vec :: with_capacity (count . min (max_initial_capacity)) }) ; for _ in 0 .. count { let input_ = input . clone () ; match self . parser . process :: < OM > (input_) { Ok ((i , o)) => { res = OM :: Output :: combine (res , o , | mut res , o | { res . push (o) ; res }) ; input = i ; } Err (Err :: Error (e)) => { return Err (Err :: Error (OM :: Error :: map (e , | e | { < F as Parser < I > > :: Error :: append (i , ErrorKind :: Count , e) }))) ; } Err (e) => { return Err (e) ; } } } Ok ((input , res)) } } } }
    };
}

impl_267!();