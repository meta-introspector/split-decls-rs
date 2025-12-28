macro_rules! deps {
    () => {
        Input!();
        OutputMode!();
        Err!();
        ParseError!();
        Parser!();
        Error!();
        AsChar!();
        Needed!();
        ErrorKind!();
        ExtendInto!();
        PResult!();
        EscapedTransform!();
        Offset!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > , F , G , ExtendItem , Output > Parser < I > for EscapedTransform < F , G , Error , ExtendItem , Output > where I : Clone + crate :: traits :: Offset + Input , I : crate :: traits :: ExtendInto < Item = ExtendItem , Extender = Output > , < F as Parser < I > > :: Output : crate :: traits :: ExtendInto < Item = ExtendItem , Extender = Output > , < G as Parser < I > > :: Output : crate :: traits :: ExtendInto < Item = ExtendItem , Extender = Output > , < I as Input > :: Item : crate :: traits :: AsChar , F : Parser < I , Error = Error > , G : Parser < I , Error = Error > , Error : ParseError < I > , { type Output = Output ; type Error = Error ; fn process < OM : OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut index = 0 ; let mut res = OM :: Output :: bind (| | input . new_builder ()) ; while index < input . input_len () { let current_len = input . input_len () ; let remainder = input . take_from (index) ; match self . normal . process :: < OM > (remainder . clone ()) { Ok ((i2 , o)) => { res = OM :: Output :: combine (o , res , | o , mut res | { o . extend_into (& mut res) ; res }) ; if i2 . input_len () == 0 { if OM :: Incomplete :: is_streaming () { return Err (Err :: Incomplete (Needed :: Unknown)) ; } else { let index = input . input_len () ; return Ok ((input . take_from (index) , res)) ; } } else if i2 . input_len () == current_len { return Ok ((remainder , res)) ; } else { index = input . offset (& i2) ; } } Err (Err :: Error (_)) => { if remainder . iter_elements () . next () . unwrap () . as_char () == self . control_char { let next = index + self . control_char . len_utf8 () ; let input_len = input . input_len () ; if next >= input_len { if OM :: Incomplete :: is_streaming () { return Err (Err :: Incomplete (Needed :: Unknown)) ; } else { return Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (remainder , ErrorKind :: EscapedTransform) }))) ; } } else { match self . transform . process :: < OM > (input . take_from (next)) { Ok ((i2 , o)) => { res = OM :: Output :: combine (o , res , | o , mut res | { o . extend_into (& mut res) ; res }) ; if i2 . input_len () == 0 { if OM :: Incomplete :: is_streaming () { return Err (Err :: Incomplete (Needed :: Unknown)) ; } else { return Ok ((input . take_from (input . input_len ()) , res)) ; } } else { index = input . offset (& i2) ; } } Err (Err :: Error (e)) => return Err (Err :: Error (e)) , Err (Err :: Failure (e)) => { return Err (Err :: Failure (e)) ; } Err (Err :: Incomplete (i)) => { return Err (Err :: Incomplete (i)) ; } } } } else { if index == 0 { return Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (remainder , ErrorKind :: EscapedTransform) }))) ; } return Ok ((remainder , res)) ; } } Err (Err :: Failure (e)) => { return Err (Err :: Failure (e)) ; } Err (Err :: Incomplete (i)) => { return Err (Err :: Incomplete (i)) ; } } } if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: Unknown)) } else { Ok ((input . take_from (index) , res)) } } }
    };
}

impl_487!();