// Generated macro for impl_62 (impl)
macro_rules! Depcrate_deserializerimpl_62 {
() => {
// Module: crate::deserializer
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , 'de : 'a , T : DeRecord < 'de > > MapAccess < 'de > for & 'a mut DeRecordWrap < T > { type Error = DeserializeError ; fn next_key_seed < K : DeserializeSeed < 'de > > (& mut self , seed : K ,) -> Result < Option < K :: Value > , Self :: Error > { assert ! (self . has_headers ()) ; let field = match self . next_header_bytes () ? { None => return Ok (None) , Some (field) => field , } ; seed . deserialize (BorrowedBytesDeserializer :: new (field)) . map (Some) } fn next_value_seed < K : DeserializeSeed < 'de > > (& mut self , seed : K ,) -> Result < K :: Value , Self :: Error > { seed . deserialize (& mut * * self) } }
};
}
