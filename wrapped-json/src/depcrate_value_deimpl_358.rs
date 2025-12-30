// Generated macro for impl_358 (impl)
macro_rules! Depcrate_value_deimpl_358 {
() => {
// Module: crate::value::de
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'de > SeqAccess < 'de > for SeqDeserializer { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Error > where T : DeserializeSeed < 'de > , { match self . iter . next () { Some (value) => seed . deserialize (value) . map (Some) , None => Ok (None) , } } fn size_hint (& self) -> Option < usize > { match self . iter . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
};
}
