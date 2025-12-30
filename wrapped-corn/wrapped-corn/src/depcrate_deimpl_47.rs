// Generated macro for impl_47 (impl)
macro_rules! Depcrate_deimpl_47 {
() => {
// Module: crate::de
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'de > de :: MapAccess < 'de > for Map < 'de > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> std :: result :: Result < Option < K :: Value > , Self :: Error > where K : DeserializeSeed < 'de > , { if let Some (value) = self . values . pop_front () { seed . deserialize (& mut Deserializer :: from_value (value)) . map (Some) } else { Ok (None) } } fn next_value_seed < V > (& mut self , seed : V) -> std :: result :: Result < V :: Value , Self :: Error > where V : DeserializeSeed < 'de > , { match self . values . pop_front () { Some (value) => seed . deserialize (& mut Deserializer :: from_value (value)) , None => Err (Error :: DeserializationError ("Expected value to exist" . to_string () ,)) , } } fn size_hint (& self) -> Option < usize > { Some (self . values . len () / 2) } }
};
}
