macro_rules! deps {
    () => {
        Decode!();
        IntegerType!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroU16 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU16 :: new (u16 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U16 , }) } }
    };
}

impl_313!();