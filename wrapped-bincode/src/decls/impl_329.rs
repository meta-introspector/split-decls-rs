macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        DecodeError!();
        IntegerType!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < Context > Decode < Context > for NonZeroUsize { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { NonZeroUsize :: new (usize :: decode (decoder) ?) . ok_or (DecodeError :: NonZeroTypeIsZero { non_zero_type : IntegerType :: Usize , }) } }
    };
}

impl_329!();