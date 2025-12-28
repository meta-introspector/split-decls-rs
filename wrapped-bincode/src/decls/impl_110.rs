macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < Context > Decode < Context > for PathBuf { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let string = std :: string :: String :: decode (decoder) ? ; Ok (string . into ()) } }
    };
}

impl_110!();