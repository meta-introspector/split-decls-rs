// Generated macro for impl_307 (impl)
macro_rules! Depcrate_valueimpl_307 {
() => {
// Module: crate::value
// Provides: {"impl_307"}
// Dependencies: {}
impl < 'a , 'de > MapAccess < 'de > for MapAccessor < 'a > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : DeserializeSeed < 'de > , { match self . items . pop () { Some ((key , value)) => { self . value = Some (value) ; seed . deserialize (key) . map (Some) } None => Ok (None) , } } # [allow (clippy :: panic)] fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : DeserializeSeed < 'de > , { match self . value . take () { Some (value) => seed . deserialize (value) , None => panic ! ("Contract violation: value before key") , } } fn size_hint (& self) -> Option < usize > { Some (self . items . len ()) } }
};
}
