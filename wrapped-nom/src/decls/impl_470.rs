macro_rules! deps {
    () => {
        OutputMode!();
        Err!();
        PResult!();
        ErrorKind!();
        TakeWhileMN!();
        Needed!();
        Error!();
        Input!();
        ParseError!();
        Parser!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < I , Error : ParseError < I > , F > Parser < I > for TakeWhileMN < F , Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { type Output = I ; type Error = Error ; fn process < OM : OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut count = 0 ; for (i , (index , item)) in input . iter_indices () . enumerate () { if i == self . n { return Ok ((input . take_from (index) , OM :: Output :: bind (| | input . take (index)) ,)) ; } if ! (self . predicate) (item) { if i >= self . m { return Ok ((input . take_from (index) , OM :: Output :: bind (| | input . take (index)) ,)) ; } else { return Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (input , ErrorKind :: TakeWhileMN) }))) ; } } count += 1 ; } let input_len = input . input_len () ; if OM :: Incomplete :: is_streaming () { let needed = if self . m > input_len { self . m - input_len } else { 1 } ; Err (Err :: Incomplete (Needed :: new (needed))) } else if count >= self . m { Ok ((input . take_from (input_len) , OM :: Output :: bind (| | input . take (input_len)) ,)) } else { Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (input , ErrorKind :: TakeWhileMN) }))) } } }
    };
}

impl_470!()