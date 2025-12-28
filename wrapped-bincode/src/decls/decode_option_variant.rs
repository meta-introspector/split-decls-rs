macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        AllowedEnumVariants!();
    };
}

macro_rules! decode_option_variant {
    () => {
        deps!();
        # [doc = " Decodes only the option variant from the decoder. Will not read any more data than that."] # [inline] pub (crate) fn decode_option_variant < D : Decoder > (decoder : & mut D , type_name : & 'static str ,) -> Result < Option < () > , DecodeError > { let is_some = u8 :: decode (decoder) ? ; match is_some { 0 => Ok (None) , 1 => Ok (Some (())) , x => Err (DecodeError :: UnexpectedVariant { found : x as u32 , allowed : & crate :: error :: AllowedEnumVariants :: Range { max : 1 , min : 0 } , type_name , }) , } }
    };
}

decode_option_variant!();