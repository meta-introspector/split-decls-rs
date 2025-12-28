macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for RwLock < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (RwLock :: new (t)) } }
    };
}

impl_102!();