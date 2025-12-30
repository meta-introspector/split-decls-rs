// Generated macro for impl_305 (impl)
macro_rules! Depcrate_valueimpl_305 {
() => {
// Module: crate::value
// Provides: {"impl_305"}
// Dependencies: {}
impl < 'a , 'de > SeqAccess < 'de > for SeqAccessor < 'a > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > > where T : DeserializeSeed < 'de > , { self . seq . pop () . map_or (Ok (None) , | v | seed . deserialize (v) . map (Some)) } fn size_hint (& self) -> Option < usize > { Some (self . seq . len ()) } }
};
}
