macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < Context > Decode < Context > for () { fn decode < D : Decoder < Context = Context > > (_ : & mut D) -> Result < Self , DecodeError > { Ok (()) } }
    };
}

impl_369!()