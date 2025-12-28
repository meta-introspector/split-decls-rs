macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Mutex < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Mutex :: new (t)) } }
    };
}

impl_99!()