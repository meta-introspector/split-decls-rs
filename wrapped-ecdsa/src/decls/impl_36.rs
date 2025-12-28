macro_rules! deps {
    () => {
        SignatureRef!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl EncodeValue for SignatureRef < '_ > { fn value_len (& self) -> der :: Result < Length > { self . r . encoded_len () ? + self . s . encoded_len () ? } fn encode_value (& self , encoder : & mut impl Writer) -> der :: Result < () > { self . r . encode (encoder) ? ; self . s . encode (encoder) ? ; Ok (()) } }
    };
}

impl_36!()