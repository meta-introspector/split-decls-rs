macro_rules! deps {
    () => {
        SalsaAttr!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl TryFrom < syn :: Attribute > for SalsaAttr { type Error = syn :: Attribute ; fn try_from (attr : syn :: Attribute) -> Result < SalsaAttr , syn :: Attribute > { if is_not_salsa_attr_path (attr . path ()) { return Err (attr) ; } let span = attr . span () ; let name = attr . path () . segments [1] . ident . to_string () ; let tts = match attr . meta { syn :: Meta :: Path (path) => path . into_token_stream () , syn :: Meta :: List (ref list) => { let tts = list . into_token_stream () . into_iter () . skip (attr . path () . to_token_stream () . into_iter () . count ()) ; proc_macro2 :: TokenStream :: from_iter (tts) } syn :: Meta :: NameValue (nv) => nv . into_token_stream () , } . into () ; Ok (SalsaAttr { name , tts , span }) } }
    };
}

impl_6!()