macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
        IntegerType!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroI128 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI128 :: new (i128 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I128 , }) } }
    };
}

impl_349!();