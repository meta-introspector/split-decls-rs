macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for RangeInclusive < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let min = T :: decode (decoder) ? ; let max = T :: decode (decoder) ? ; Ok (RangeInclusive :: new (min , max)) } }
    };
}

impl_385!();