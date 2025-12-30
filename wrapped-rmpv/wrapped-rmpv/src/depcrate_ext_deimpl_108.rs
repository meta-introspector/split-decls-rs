// Generated macro for impl_108 (impl)
macro_rules! Depcrate_ext_deimpl_108 {
() => {
// Module: crate::ext::de
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'de , I , U > SeqAccess < 'de > for SeqDeserializer < I > where I : Iterator < Item = U > , U : Deserializer < 'de , Error = Error > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : de :: DeserializeSeed < 'de > { match self . iter . next () { Some (val) => seed . deserialize (val) . map (Some) , None => Ok (None) , } } }
};
}
