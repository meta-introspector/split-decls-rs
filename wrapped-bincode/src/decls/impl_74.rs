macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Rc < [T] > where T : Decode < Context > + 'static , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = Vec :: decode (decoder) ? ; Ok (vec . into ()) } }
    };
}

impl_74!();