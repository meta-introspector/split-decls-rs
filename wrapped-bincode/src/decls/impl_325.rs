macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        IntegerType!();
        Decode!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroU128 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroU128 :: new (u128 :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: U128 , }) } }
    };
}

impl_325!()