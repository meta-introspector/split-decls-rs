macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        IntegerType!();
        Decode!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroI16 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI16 :: new (i16 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I16 , }) } }
    };
}

impl_337!()