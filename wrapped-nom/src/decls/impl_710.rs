macro_rules! deps {
    () => {
        Err!();
        Parser!();
        Error!();
        Input!();
        PResult!();
        Needed!();
        ParseError!();
        LeUint!();
        OutputMode!();
        ErrorKind!();
    };
}

macro_rules! impl_710 {
    () => {
        deps!();
        impl < I , Uint , E : ParseError < I > > Parser < I > for LeUint < Uint , E > where I : Input < Item = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , { type Output = Uint ; type Error = E ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { if input . input_len () < self . bound { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: new (self . bound - input . input_len ()))) } else { Err (Err :: Error (OM :: Error :: bind (| | { make_error (input , ErrorKind :: Eof) }))) } } else { let res = OM :: Output :: bind (| | { let mut res = Uint :: default () ; for (index , byte) in input . iter_elements () . take (self . bound) . enumerate () { res = res + (Uint :: from (byte) << (8 * index as u8)) ; } res }) ; Ok ((input . take_from (self . bound) , res)) } } }
    };
}

impl_710!()