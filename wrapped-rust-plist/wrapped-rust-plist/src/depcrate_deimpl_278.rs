// Generated macro for impl_278 (impl)
macro_rules! Depcrate_deimpl_278 {
() => {
// Module: crate::de
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'de , 'a , 'event , I > de :: MapAccess < 'de > for MapAndSeqAccess < 'a , 'event , I > where I : 'a + IntoIterator < Item = Result < Event < 'event > , Error > > , { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : de :: DeserializeSeed < 'de > , { if let Some (& Ok (Event :: EndCollection)) = self . de . events . peek () { return Ok (None) ; } self . remaining = self . remaining . map (| r | r . saturating_sub (1)) ; self . de . with_option_mode (OptionMode :: Explicit , | this | seed . deserialize (this)) . map (Some) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : de :: DeserializeSeed < 'de > , { let option_mode = if self . is_struct { OptionMode :: StructField } else { OptionMode :: Explicit } ; self . de . with_option_mode (option_mode , | this | seed . deserialize (this)) } fn size_hint (& self) -> Option < usize > { self . remaining } }
};
}
