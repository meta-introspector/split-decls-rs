macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for RefCell < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (RefCell :: new (t)) } }
    };
}

impl_379!();