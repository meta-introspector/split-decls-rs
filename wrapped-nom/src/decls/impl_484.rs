macro_rules! deps {
    () => {
        Err!();
        AsChar!();
        ParseError!();
        OutputMode!();
        Escaped!();
        PResult!();
        Offset!();
        Error!();
        OutputM!();
        Check!();
        ErrorKind!();
        Parser!();
        Needed!();
        Input!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > , F , G > Parser < I > for Escaped < F , G , Error > where I : Input + Clone + crate :: traits :: Offset , < I as Input > :: Item : crate :: traits :: AsChar , F : Parser < I , Error = Error > , G : Parser < I , Error = Error > , Error : ParseError < I > , { type Output = I ; type Error = Error ; fn process < OM : OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut i = input . clone () ; while i . input_len () > 0 { let current_len = i . input_len () ; match self . normal . process :: < OutputM < Check , Check , OM :: Incomplete > > (i . clone ()) { Ok ((i2 , _)) => { if i2 . input_len () == 0 { if OM :: Incomplete :: is_streaming () { return Err (Err :: Incomplete (Needed :: Unknown)) ; } else { let index = input . input_len () ; return Ok ((input . take_from (index) , OM :: Output :: bind (| | input . take (index)) ,)) ; } } else if i2 . input_len () == current_len { let index = input . offset (& i2) ; return Ok ((input . take_from (index) , OM :: Output :: bind (| | input . take (index)) ,)) ; } else { i = i2 ; } } Err (Err :: Error (_)) => { if i . iter_elements () . next () . unwrap () . as_char () == self . control_char { let next = self . control_char . len_utf8 () ; if next >= i . input_len () { if OM :: Incomplete :: is_streaming () { return Err (Err :: Incomplete (Needed :: new (1))) ; } else { return Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (input , ErrorKind :: Escaped) }))) ; } } else { match self . escapable . process :: < OutputM < Check , OM :: Error , OM :: Incomplete > > (i . take_from (next)) { Ok ((i2 , _)) => { if i2 . input_len () == 0 { if OM :: Incomplete :: is_streaming () { return Err (Err :: Incomplete (Needed :: Unknown)) ; } else { let index = input . input_len () ; return Ok ((input . take_from (index) , OM :: Output :: bind (| | input . take (index)) ,)) ; } } else { i = i2 ; } } Err (e) => return Err (e) , } } } else { let index = input . offset (& i) ; if index == 0 { return Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (input , ErrorKind :: Escaped) }))) ; } else { return Ok ((input . take_from (index) , OM :: Output :: bind (| | input . take (index)) ,)) ; } } } Err (Err :: Failure (e)) => { return Err (Err :: Failure (e)) ; } Err (Err :: Incomplete (i)) => { return Err (Err :: Incomplete (i)) ; } } } if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: Unknown)) } else { let index = input . input_len () ; Ok ((input . take_from (index) , OM :: Output :: bind (| | input . take (index)) ,)) } } }
    };
}

impl_484!();