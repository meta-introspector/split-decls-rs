macro_rules! deps {
    () => {
        Value!();
        MapAccess!();
        Result!();
        Error!();
        NumberFieldDeserializer!();
        NumberDeserializer!();
    };
}

macro_rules! impl_555 {
    () => {
        deps!();
        # [cfg (feature = "arbitrary_precision")] impl < 'de > MapAccess < 'de > for NumberDeserializer { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : de :: DeserializeSeed < 'de > , { if self . number . is_none () { return Ok (None) ; } seed . deserialize (NumberFieldDeserializer) . map (Some) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (self . number . take () . unwrap () . into_deserializer ()) } }
    };
}

impl_555!();