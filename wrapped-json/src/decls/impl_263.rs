macro_rules! deps {
    () => {
        SeqDeserializer!();
        SeqAccess!();
        Value!();
        Result!();
        Error!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < 'de > SeqAccess < 'de > for SeqDeserializer { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Error > where T : DeserializeSeed < 'de > , { match self . iter . next () { Some (value) => seed . deserialize (value) . map (Some) , None => Ok (None) , } } fn size_hint (& self) -> Option < usize > { match self . iter . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
    };
}

impl_263!();