macro_rules! deps {
    () => {
        SerdeDecoder!();
        Decoder!();
        DecodeError!();
        OwnedSerdeDecoder!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < DE : Decoder > OwnedSerdeDecoder < DE > { # [doc = " Return a type implementing `serde::Deserializer`."] pub fn as_deserializer < 'a > (& 'a mut self ,) -> impl for < 'de > serde :: Deserializer < 'de , Error = DecodeError > + 'a { SerdeDecoder { de : & mut self . de } } }
    };
}

impl_149!()