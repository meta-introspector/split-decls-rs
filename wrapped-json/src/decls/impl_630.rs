macro_rules! deps {
    () => {
        RawKeyDeserializer!();
        MapAccess!();
        Error!();
        Result!();
        OwnedRawDeserializer!();
        Value!();
    };
}

macro_rules! impl_630 {
    () => {
        deps!();
        impl < 'de > MapAccess < 'de > for OwnedRawDeserializer { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : de :: DeserializeSeed < 'de > , { if self . raw_value . is_none () { return Ok (None) ; } seed . deserialize (RawKeyDeserializer) . map (Some) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (self . raw_value . take () . unwrap () . into_deserializer ()) } }
    };
}

impl_630!();