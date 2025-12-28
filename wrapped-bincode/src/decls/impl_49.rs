macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for VecDeque < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: decode (decoder) ? . into ()) } }
    };
}

impl_49!()