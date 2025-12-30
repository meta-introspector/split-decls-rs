// Generated macro for impl_32 (impl)
macro_rules! Depcrate_deimpl_32 {
() => {
// Module: crate::de
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'de > de :: MapAccess < 'de > for Map < 'de > { type Error = Error ; fn size_hint (& self) -> Option < usize > { Some (self . pairs . len () / 2) } fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : de :: DeserializeSeed < 'de > , { if let Some (pair) = self . pairs . pop_front () { seed . deserialize (& mut Deserializer :: from_pair (pair)) . map (Some) } else { Ok (None) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (& mut Deserializer :: from_pair (self . pairs . pop_front () . unwrap () ,)) } }
};
}
