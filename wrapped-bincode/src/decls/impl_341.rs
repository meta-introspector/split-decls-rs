macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
        IntegerType!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroI32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroI32 :: new (i32 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: I32 , }) } }
    };
}

impl_341!()