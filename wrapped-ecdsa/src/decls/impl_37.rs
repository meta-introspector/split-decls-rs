macro_rules! deps {
    () => {
        SignatureRef!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a > DecodeValue < 'a > for SignatureRef < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { Ok (Self { r : UintRef :: decode (reader) ? , s : UintRef :: decode (reader) ? , }) } }
    };
}

impl_37!()