macro_rules! deps {
    () => {
        Decoder!();
        Decode!();
        AllowedEnumVariants!();
        DecodeError!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < Context > Decode < Context > for IpAddr { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match u32 :: decode (decoder) ? { 0 => Ok (IpAddr :: V4 (Ipv4Addr :: decode (decoder) ?)) , 1 => Ok (IpAddr :: V6 (Ipv6Addr :: decode (decoder) ?)) , found => Err (DecodeError :: UnexpectedVariant { allowed : & crate :: error :: AllowedEnumVariants :: Range { min : 0 , max : 1 } , found , type_name : core :: any :: type_name :: < IpAddr > () , }) , } } }
    };
}

impl_113!()