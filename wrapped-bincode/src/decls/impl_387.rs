macro_rules! deps {
    () => {
        Decode!();
        Decoder!();
        AllowedEnumVariants!();
        DecodeError!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl < T , Context > Decode < Context > for Bound < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match u32 :: decode (decoder) ? { 0 => Ok (Bound :: Unbounded) , 1 => Ok (Bound :: Included (T :: decode (decoder) ?)) , 2 => Ok (Bound :: Excluded (T :: decode (decoder) ?)) , x => Err (DecodeError :: UnexpectedVariant { allowed : & crate :: error :: AllowedEnumVariants :: Range { max : 2 , min : 0 } , found : x , type_name : core :: any :: type_name :: < Bound < T > > () , }) , } } }
    };
}

impl_387!()