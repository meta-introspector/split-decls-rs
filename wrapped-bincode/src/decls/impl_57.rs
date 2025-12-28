macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < Context > Decode < Context > for Box < str > { fn decode < D : Decoder > (decoder : & mut D) -> Result < Self , DecodeError > { String :: decode (decoder) . map (String :: into_boxed_str) } }
    };
}

impl_57!();