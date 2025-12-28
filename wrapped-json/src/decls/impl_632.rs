macro_rules! deps {
    () => {
        Result!();
        Error!();
        Value!();
        RawKeyDeserializer!();
        BorrowedRawDeserializer!();
        MapAccess!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl < 'de > MapAccess < 'de > for BorrowedRawDeserializer < 'de > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : de :: DeserializeSeed < 'de > , { if self . raw_value . is_none () { return Ok (None) ; } seed . deserialize (RawKeyDeserializer) . map (Some) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (BorrowedStrDeserializer :: new (self . raw_value . take () . unwrap ())) } }
    };
}

impl_632!();