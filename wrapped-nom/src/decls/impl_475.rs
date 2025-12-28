macro_rules! deps {
    () => {
        OutputMode!();
        ErrorKind!();
        ParseError!();
        Take!();
        Parser!();
        Err!();
        Error!();
        PResult!();
        Input!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > > Parser < I > for Take < Error > where I : Input , { type Output = I ; type Error = Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match i . slice_index (self . length) { Err (needed) => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (needed)) } else { Err (Err :: Error (OM :: Error :: bind (| | { let e : ErrorKind = ErrorKind :: Eof ; Error :: from_error_kind (i , e) }))) } } Ok (index) => Ok ((i . take_from (index) , OM :: Output :: bind (| | i . take (index)))) , } } }
    };
}

impl_475!();