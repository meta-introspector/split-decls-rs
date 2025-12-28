macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < Context , T : Decode < Context > > Decode < Context > for Wrapping < T > { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Wrapping (T :: decode (decoder) ?)) } }
    };
}

impl_359!()