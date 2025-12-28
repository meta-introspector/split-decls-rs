macro_rules! deps {
    () => {
        OutputMode!();
        Fold!();
        PResult!();
        ErrorKind!();
        Input!();
        Parser!();
        Error!();
        NomRange!();
        Err!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < I , F , G , H , Range , Res > Parser < I > for Fold < F , G , H , Range > where I : Clone + Input , F : Parser < I > , G : FnMut (Res , < F as Parser < I > > :: Output) -> Res , H : FnMut () -> Res , Range : NomRange < usize > , { type Output = Res ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { if self . range . is_inverted () { return Err (Err :: Failure (< F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Fold ,))) ; } let mut acc = OM :: Output :: bind (| | (self . init) ()) ; for count in self . range . saturating_iter () { let len = input . input_len () ; match self . parser . process :: < OM > (input . clone ()) { Ok ((tail , value)) => { if tail . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (tail , ErrorKind :: Fold) }))) ; } acc = OM :: Output :: combine (acc , value , | acc , value | (self . fold) (acc , value)) ; input = tail ; } Err (Err :: Error (err)) => { if ! self . range . contains (& count) { return Err (Err :: Error (OM :: Error :: map (err , | err | { < F as Parser < I > > :: Error :: append (input , ErrorKind :: Fold , err) }))) ; } else { break ; } } Err (e) => return Err (e) , } } Ok ((input , acc)) } }
    };
}

impl_273!()