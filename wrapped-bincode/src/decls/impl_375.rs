macro_rules! deps {
    () => {
        DecodeError!();
        Decode!();
        AllowedEnumVariants!();
        Decoder!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl < Context , T , U > Decode < Context > for Result < T , U > where T : Decode < Context > , U : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let is_ok = u32 :: decode (decoder) ? ; match is_ok { 0 => { let t = T :: decode (decoder) ? ; Ok (Ok (t)) } 1 => { let u = U :: decode (decoder) ? ; Ok (Err (u)) } x => Err (DecodeError :: UnexpectedVariant { found : x , allowed : & crate :: error :: AllowedEnumVariants :: Range { max : 1 , min : 0 } , type_name : core :: any :: type_name :: < Result < T , U > > () , }) , } } }
    };
}

impl_375!();