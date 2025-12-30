// Generated macro for impl_50 (impl)
macro_rules! Depcrate_deimpl_50 {
() => {
// Module: crate::de
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'de > de :: SeqAccess < 'de > for Seq < 'de > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T ,) -> std :: result :: Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > , { if let Some (value) = self . values . pop_front () { seed . deserialize (& mut Deserializer :: from_value (value)) . map (Some) } else { Ok (None) } } fn size_hint (& self) -> Option < usize > { Some (self . values . len ()) } }
};
}
