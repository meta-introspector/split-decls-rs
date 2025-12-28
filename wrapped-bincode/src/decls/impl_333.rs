macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
        IntegerType!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroI8 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI8 :: new (i8 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I8 , }) } }
    };
}

impl_333!()