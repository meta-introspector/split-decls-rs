macro_rules! deps {
    () => {
        DecodeError!();
        IntegerType!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroU32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU32 :: new (u32 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U32 , }) } }
    };
}

impl_317!()