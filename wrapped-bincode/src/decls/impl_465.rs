macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < T > Encode for [T] where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { super :: encode_slice_len (encoder , self . len ()) ? ; if unty :: type_equal :: < T , u8 > () { let t : & [u8] = unsafe { core :: mem :: transmute (self) } ; encoder . writer () . write (t) ? ; return Ok (()) ; } for item in self { item . encode (encoder) ? ; } Ok (()) } }
    };
}

impl_465!()