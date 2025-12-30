// Generated macro for impl_121 (impl)
macro_rules! Depcrate_ext_deimpl_121 {
() => {
// Module: crate::ext::de
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'de > de :: MapAccess < 'de > for MapRefDeserializer < 'de > { type Error = Error ; fn next_key_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > { match self . iter . next () { Some ((key , val)) => { self . val = Some (val) ; seed . deserialize (key) . map (Some) } None => Ok (None) , } } fn next_value_seed < T > (& mut self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > { match self . val . take () { Some (val) => seed . deserialize (val) , None => Err (de :: Error :: custom ("value is missing")) , } } }
};
}
