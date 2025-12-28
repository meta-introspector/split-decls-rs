macro_rules! deps {
    () => {
        Needed!();
        TakeUntil!();
        Parser!();
        Error!();
        OutputMode!();
        FindSubstring!();
        Input!();
        ErrorKind!();
        ParseError!();
        PResult!();
        Err!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < I , T , Error : ParseError < I > > Parser < I > for TakeUntil < T , Error > where I : Input + FindSubstring < T > , T : Clone , { type Output = I ; type Error = Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match i . find_substring (self . tag . clone ()) { None => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: Unknown)) } else { Err (Err :: Error (OM :: Error :: bind (| | { let e : ErrorKind = ErrorKind :: TakeUntil ; Error :: from_error_kind (i , e) }))) } } Some (index) => Ok ((i . take_from (index) , OM :: Output :: bind (| | i . take (index)))) , } } }
    };
}

impl_478!()