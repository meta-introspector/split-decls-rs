macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Cell < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Cell :: new (t)) } }
    };
}

impl_377!();