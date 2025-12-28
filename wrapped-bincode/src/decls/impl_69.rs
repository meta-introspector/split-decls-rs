macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Rc < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Rc :: new (t)) } }
    };
}

impl_69!()