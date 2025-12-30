// Generated macro for impl_50 (impl)
macro_rules! Depcrate_deimpl_50 {
() => {
// Module: crate::de
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'de > de :: MapAccess < 'de > for MapAccess { type Error = ConfigError ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : de :: DeserializeSeed < 'de > , { if let Some ((ref key_s , _)) = self . elements . front () { let key_de = Value :: new (None , key_s as & str) ; let key = de :: DeserializeSeed :: deserialize (seed , key_de) ? ; Ok (Some (key)) } else { Ok (None) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : de :: DeserializeSeed < 'de > , { let (key , value) = self . elements . pop_front () . unwrap () ; de :: DeserializeSeed :: deserialize (seed , value) . map_err (| e | e . prepend_key (& key)) } }
};
}
