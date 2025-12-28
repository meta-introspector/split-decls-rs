macro_rules! deps {
    () => {
        ParseError!();
        Emit!();
        OutputM!();
        Needed!();
        Err!();
        ErrorKind!();
        PResult!();
        ToUsize!();
        OutputMode!();
        LengthValue!();
        Parser!();
        Complete!();
        Input!();
        Error!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < I , F , G , E > Parser < I > for LengthValue < F , G , E > where I : Clone + Input , F : Parser < I , Error = E > , G : Parser < I , Error = E > , < F as Parser < I > > :: Output : ToUsize , E : ParseError < I > , { type Output = < G as Parser < I > > :: Output ; type Error = E ; fn process < OM : OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let (i , length) = self . length . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (input) ? ; let length : usize = length . to_usize () ; if let Some (needed) = length . checked_sub (i . input_len ()) . and_then (NonZeroUsize :: new) { Err (Err :: Incomplete (Needed :: Size (needed))) } else { let (rest , i) = i . take_split (length) ; match self . parser . process :: < OM > (i . clone ()) { Err (Err :: Incomplete (_)) => Err (Err :: Error (OM :: Error :: bind (| | { E :: from_error_kind (i , ErrorKind :: Complete) }))) , Err (e) => Err (e) , Ok ((_ , o)) => Ok ((rest , o)) , } } } }
    };
}

impl_264!()