macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
        IntegerType!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroU8 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU8 :: new (u8 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U8 , }) } }
    };
}

impl_309!();