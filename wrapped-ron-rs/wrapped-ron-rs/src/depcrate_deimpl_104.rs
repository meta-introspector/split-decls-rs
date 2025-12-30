// Generated macro for impl_104 (impl)
macro_rules! Depcrate_deimpl_104 {
() => {
// Module: crate::de
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'de , 'a > de :: MapAccess < 'de > for SerdeEnumContent < 'a , 'de > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : DeserializeSeed < 'de > , { self . ident . take () . map (| ident | seed . deserialize (serde :: de :: value :: StrDeserializer :: new (ident))) . transpose () } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : DeserializeSeed < 'de > , { self . de . parser . skip_ws () ? ; let old_serde_content_newtype = self . de . serde_content_newtype ; self . de . serde_content_newtype = true ; let result = seed . deserialize (& mut * self . de) ; self . de . serde_content_newtype = old_serde_content_newtype ; result } }
};
}
