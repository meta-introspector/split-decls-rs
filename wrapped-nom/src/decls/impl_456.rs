macro_rules! deps {
    () => {
        Compare!();
        Error!();
        Tag!();
        Input!();
        OutputMode!();
        CompareResult!();
        ErrorKind!();
        ParseError!();
        Parser!();
        PResult!();
        Needed!();
        Err!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > , T > Parser < I > for Tag < T , Error > where I : Input + Compare < T > , T : Input + Clone , { type Output = I ; type Error = Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let tag_len = self . tag . input_len () ; let t = self . tag . clone () ; match i . compare (t) { CompareResult :: Ok => Ok ((i . take_from (tag_len) , OM :: Output :: bind (| | i . take (tag_len)))) , CompareResult :: Incomplete => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: new (tag_len - i . input_len ()))) } else { Err (Err :: Error (OM :: Error :: bind (| | { let e : ErrorKind = ErrorKind :: Tag ; Error :: from_error_kind (i , e) }))) } } CompareResult :: Error => Err (Err :: Error (OM :: Error :: bind (| | { let e : ErrorKind = ErrorKind :: Tag ; Error :: from_error_kind (i , e) }))) , } } }
    };
}

impl_456!();