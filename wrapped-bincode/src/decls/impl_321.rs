macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        Decoder!();
        IntegerType!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroU64 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU64 :: new (u64 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U64 , }) } }
    };
}

impl_321!();