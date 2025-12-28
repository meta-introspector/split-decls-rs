macro_rules! deps {
    () => {
        ObjectIdentifier!();
        Encoder!();
        Error!();
        Parser!();
        Result!();
        Arc!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl Parser { # [doc = " Parse an OID from a dot-delimited string e.g. `1.2.840.113549.1.1.1`"] pub (crate) const fn parse (s : & str) -> Result < Self > { let bytes = s . as_bytes () ; if bytes . is_empty () { return Err (Error :: Empty) ; } match bytes [0] { b'0' ..= b'9' => Self { current_arc : None , encoder : Encoder :: new () , } . parse_bytes (bytes) , actual => Err (Error :: DigitExpected { actual }) , } } # [doc = " Finish parsing, returning the result"] pub (crate) const fn finish (self) -> Result < ObjectIdentifier > { self . encoder . finish () } # [doc = " Parse the remaining bytes"] const fn parse_bytes (mut self , bytes : & [u8]) -> Result < Self > { match bytes { [] => match self . current_arc { Some (arc) => match self . encoder . arc (arc) { Ok (encoder) => { self . encoder = encoder ; Ok (self) } Err (err) => Err (err) , } , None => Err (Error :: TrailingDot) , } , [byte @ b'0' ..= b'9' , remaining @ ..] => { let digit = byte . saturating_sub (b'0') ; let arc = match self . current_arc { Some (arc) => arc , None => 0 , } ; self . current_arc = match arc . checked_mul (10) { Some (arc) => match arc . checked_add (digit as Arc) { None => return Err (Error :: ArcTooBig) , Some (arc) => Some (arc) , } , None => return Err (Error :: ArcTooBig) , } ; self . parse_bytes (remaining) } [b'.' , remaining @ ..] => { match self . current_arc { Some (arc) => { if remaining . is_empty () { return Err (Error :: TrailingDot) ; } match self . encoder . arc (arc) { Ok (encoder) => { self . encoder = encoder ; self . current_arc = None ; self . parse_bytes (remaining) } Err (err) => Err (err) , } } None => Err (Error :: RepeatedDot) , } } [byte , ..] => Err (Error :: DigitExpected { actual : * byte }) , } } }
    };
}

impl_35!();