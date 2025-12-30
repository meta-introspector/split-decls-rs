// Generated macro for impl_29 (impl)
macro_rules! Depcrate_deimpl_29 {
() => {
// Module: crate::de
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'de > de :: SeqAccess < 'de > for Seq < 'de > { type Error = Error ; fn size_hint (& self) -> Option < usize > { Some (self . pairs . len ()) } fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > > where T : de :: DeserializeSeed < 'de > , { if let Some (pair) = self . pairs . pop_front () { seed . deserialize (& mut Deserializer :: from_pair (pair)) . map (Some) } else { Ok (None) } } }
};
}
