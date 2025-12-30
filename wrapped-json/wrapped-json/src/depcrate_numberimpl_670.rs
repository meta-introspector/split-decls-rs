// Generated macro for impl_670 (impl)
macro_rules! Depcrate_numberimpl_670 {
() => {
// Module: crate::number
// Provides: {"impl_670"}
// Dependencies: {}
# [cfg (feature = "arbitrary_precision")] impl < 'de > MapAccess < 'de > for NumberDeserializer { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : de :: DeserializeSeed < 'de > , { if self . number . is_none () { return Ok (None) ; } seed . deserialize (NumberFieldDeserializer) . map (Some) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (self . number . take () . unwrap () . into_deserializer ()) } }
};
}
