macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Box < [T] > where T : Decode < Context > + 'static , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = Vec :: decode (decoder) ? ; Ok (vec . into_boxed_slice ()) } }
    };
}

impl_63!()