macro_rules! deps {
    () => {
        BeUint!();
        Parser!();
        Input!();
        Error!();
        ParseError!();
        PResult!();
        Needed!();
        OutputMode!();
        Err!();
        ErrorKind!();
    };
}

macro_rules! impl_695 {
    () => {
        deps!();
        impl < I , Uint , E : ParseError < I > > Parser < I > for BeUint < Uint , E > where I : Input < Item = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , { type Output = Uint ; type Error = E ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { if input . input_len () < self . bound { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: new (self . bound - input . input_len ()))) } else { Err (Err :: Error (OM :: Error :: bind (| | { make_error (input , ErrorKind :: Eof) }))) } } else { let res = OM :: Output :: bind (| | { let mut res = Uint :: default () ; if self . bound > 1 { for byte in input . iter_elements () . take (self . bound) { res = (res << 8) + byte . into () ; } } else { for byte in input . iter_elements () . take (self . bound) { res = byte . into () ; } } res }) ; Ok ((input . take_from (self . bound) , res)) } } }
    };
}

impl_695!()