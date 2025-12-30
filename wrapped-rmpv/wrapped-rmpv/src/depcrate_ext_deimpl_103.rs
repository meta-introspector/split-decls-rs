// Generated macro for impl_103 (impl)
macro_rules! Depcrate_ext_deimpl_103 {
() => {
// Module: crate::ext::de
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'de > SeqAccess < 'de > for ExtDeserializer < 'de > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Error > where T : DeserializeSeed < 'de > , { if self . tag . is_some () || self . data . is_some () { return Ok (Some (seed . deserialize (self) ?)) ; } Ok (None) } }
};
}
