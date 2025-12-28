macro_rules! deps {
    () => {
        BorrowedSerdeDecoder!();
        DecodeError!();
        BorrowDecoder!();
        SerdeDecoder!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'de , DE : BorrowDecoder < 'de > > BorrowedSerdeDecoder < 'de , DE > { # [doc = " Return a type implementing `serde::Deserializer`."] pub fn as_deserializer < 'a > (& 'a mut self ,) -> impl serde :: Deserializer < 'de , Error = DecodeError > + 'a { SerdeDecoder { de : & mut self . de , pd : PhantomData , } } }
    };
}

impl_139!()