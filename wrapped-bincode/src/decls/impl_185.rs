macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
        SerdeDecoder!();
        Compat!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < Context , T > crate :: Decode < Context > for Compat < T > where T : serde :: de :: DeserializeOwned , { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { let serde_decoder = de_owned :: SerdeDecoder { de : decoder } ; T :: deserialize (serde_decoder) . map (Compat) } }
    };
}

impl_185!();