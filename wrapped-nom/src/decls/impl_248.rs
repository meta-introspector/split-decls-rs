macro_rules! deps {
    () => {
        Parser!();
        PResult!();
        Err!();
        OutputMode!();
        ErrorKind!();
        Error!();
        Count!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I , F > Parser < I > for Count < F > where I : Clone , F : Parser < I > , { type Output = Vec < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut input = i . clone () ; let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES / crate :: lib :: std :: mem :: size_of :: < < F as Parser < I > > :: Output > () . max (1) ; let mut res = OM :: Output :: bind (| | { crate :: lib :: std :: vec :: Vec :: with_capacity (self . count . min (max_initial_capacity)) }) ; for _ in 0 .. self . count { let input_ = input . clone () ; match self . parser . process :: < OM > (input_) { Ok ((i , o)) => { res = OM :: Output :: combine (res , o , | mut res , o | { res . push (o) ; res }) ; input = i ; } Err (Err :: Error (e)) => { return Err (Err :: Error (OM :: Error :: map (e , | e | { < F as Parser < I > > :: Error :: append (i , ErrorKind :: Count , e) }))) ; } Err (e) => { return Err (e) ; } } } Ok ((input , res)) } }
    };
}

impl_248!();