macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < Context > Decode < Context > for String { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let bytes = Vec :: < u8 > :: decode (decoder) ? ; String :: from_utf8 (bytes) . map_err (| e | DecodeError :: Utf8 { inner : e . utf8_error () , }) } }
    };
}

impl_55!();