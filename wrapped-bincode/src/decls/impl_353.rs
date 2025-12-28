macro_rules! deps {
    () => {
        Decoder!();
        IntegerType!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroIsize { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroIsize :: new (isize :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: Isize , }) } }
    };
}

impl_353!()