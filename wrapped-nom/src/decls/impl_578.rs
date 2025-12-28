macro_rules! deps {
    () => {
        AnyChar!();
        Needed!();
        Err!();
        PResult!();
        ParseError!();
        Parser!();
        Error!();
        ErrorKind!();
        Input!();
        AsChar!();
        OutputMode!();
    };
}

macro_rules! impl_578 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > > Parser < I > for AnyChar < Error > where I : Input , < I as Input > :: Item : AsChar , { type Output = char ; type Error = Error ; fn process < OM : crate :: OutputMode > (& mut self , i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match (i) . iter_elements () . next () { None => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: new (1))) } else { Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (i , ErrorKind :: Eof) }))) } } Some (c) => Ok ((i . take_from (c . len ()) , OM :: Output :: bind (| | c . as_char ()))) , } } }
    };
}

impl_578!();