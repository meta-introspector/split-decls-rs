macro_rules! deps {
    () => {
        IntegerType!();
        Decoder!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroI64 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI64 :: new (i64 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I64 , }) } }
    };
}

impl_345!();