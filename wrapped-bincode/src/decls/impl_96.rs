macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < Context > Decode < Context > for CString { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = std :: vec :: Vec :: decode (decoder) ? ; CString :: new (vec) . map_err (| inner | DecodeError :: CStringNulError { position : inner . nul_position () , }) } }
    };
}

impl_96!()