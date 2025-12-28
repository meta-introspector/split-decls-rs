macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
        AllowedEnumVariants!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < Context > Decode < Context > for SocketAddr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match u32 :: decode (decoder) ? { 0 => Ok (SocketAddr :: V4 (SocketAddrV4 :: decode (decoder) ?)) , 1 => Ok (SocketAddr :: V6 (SocketAddrV6 :: decode (decoder) ?)) , found => Err (DecodeError :: UnexpectedVariant { allowed : & crate :: error :: AllowedEnumVariants :: Range { min : 0 , max : 1 } , found , type_name : core :: any :: type_name :: < SocketAddr > () , }) , } } }
    };
}

impl_122!()