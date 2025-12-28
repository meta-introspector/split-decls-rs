macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl < Context , T > Decode < Context > for Option < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match super :: decode_option_variant (decoder , core :: any :: type_name :: < Option < T > > ()) ? { Some (_) => { let val = T :: decode (decoder) ? ; Ok (Some (val)) } None => Ok (None) , } } }
    };
}

impl_373!();