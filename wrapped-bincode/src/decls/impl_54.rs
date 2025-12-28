macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < T > Encode for Vec < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; if unty :: type_equal :: < T , u8 > () { let slice : & [u8] = unsafe { core :: mem :: transmute (self . as_slice ()) } ; encoder . writer () . write (slice) ? ; Ok (()) } else { for item in self . iter () { item . encode (encoder) ? ; } Ok (()) } } }
    };
}

impl_54!()