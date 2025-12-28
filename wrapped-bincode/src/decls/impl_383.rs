macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Range < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let min = T :: decode (decoder) ? ; let max = T :: decode (decoder) ? ; Ok (min .. max) } }
    };
}

impl_383!()