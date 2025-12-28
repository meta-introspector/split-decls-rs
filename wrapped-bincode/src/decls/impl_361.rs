macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl < Context , T : Decode < Context > > Decode < Context > for Reverse < T > { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Reverse (T :: decode (decoder) ?)) } }
    };
}

impl_361!();