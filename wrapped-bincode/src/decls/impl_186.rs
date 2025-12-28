macro_rules! deps {
    () => {
        Compat!();
        DecodeError!();
        SerdeDecoder!();
        BorrowDecode!();
        BorrowDecoder!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'de , T , Context > crate :: BorrowDecode < 'de , Context > for Compat < T > where T : serde :: de :: DeserializeOwned , { fn borrow_decode < D : crate :: de :: BorrowDecoder < 'de > > (decoder : & mut D ,) -> Result < Self , crate :: error :: DecodeError > { let serde_decoder = de_owned :: SerdeDecoder { de : decoder } ; T :: deserialize (serde_decoder) . map (Compat) } }
    };
}

impl_186!();