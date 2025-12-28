macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Box < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Box :: new (t)) } }
    };
}

impl_60!();