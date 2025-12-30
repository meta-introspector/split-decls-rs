// Generated macro for impl_277 (impl)
macro_rules! Depcrate_deimpl_277 {
() => {
// Module: crate::de
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'de , 'a , 'event , I > de :: SeqAccess < 'de > for MapAndSeqAccess < 'a , 'event , I > where I : 'a + IntoIterator < Item = Result < Event < 'event > , Error > > , { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Error > where T : de :: DeserializeSeed < 'de > , { if let Some (& Ok (Event :: EndCollection)) = self . de . events . peek () { return Ok (None) ; } self . remaining = self . remaining . map (| r | r . saturating_sub (1)) ; self . de . with_option_mode (OptionMode :: Explicit , | this | seed . deserialize (this)) . map (Some) } fn size_hint (& self) -> Option < usize > { self . remaining } }
};
}
