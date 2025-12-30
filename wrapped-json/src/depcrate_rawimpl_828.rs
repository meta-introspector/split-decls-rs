// Generated macro for impl_828 (impl)
macro_rules! Depcrate_rawimpl_828 {
() => {
// Module: crate::raw
// Provides: {"impl_828"}
// Dependencies: {}
impl < 'de > MapAccess < 'de > for OwnedRawDeserializer { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : de :: DeserializeSeed < 'de > , { if self . raw_value . is_none () { return Ok (None) ; } seed . deserialize (RawKeyDeserializer) . map (Some) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (self . raw_value . take () . unwrap () . into_deserializer ()) } }
};
}
