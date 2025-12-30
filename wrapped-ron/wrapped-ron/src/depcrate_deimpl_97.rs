// Generated macro for impl_97 (impl)
macro_rules! Depcrate_deimpl_97 {
() => {
// Module: crate::de
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'de , 'a > de :: MapAccess < 'de > for CommaSeparated < 'a , 'de > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : DeserializeSeed < 'de > , { if self . has_element () ? { self . inside_internally_tagged_enum = is_serde_tag_or_content :: < K :: Value > () ; match self . terminator { Terminator :: Struct => guard_recursion ! { self . de => seed . deserialize (& mut id :: Deserializer :: new (& mut * self . de , false)) . map (Some) } , Terminator :: MapAsStruct => guard_recursion ! { self . de => seed . deserialize (& mut id :: Deserializer :: new (& mut * self . de , true)) . map (Some) } , _ => guard_recursion ! { self . de => seed . deserialize (& mut * self . de) . map (Some) } , } } else { Ok (None) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : DeserializeSeed < 'de > , { self . de . parser . skip_ws () ? ; if self . de . parser . consume_char (':') { self . de . parser . skip_ws () ? ; let res = if self . inside_internally_tagged_enum && ! is_serde_content :: < V :: Value > () { guard_recursion ! { self . de => seed . deserialize (& mut tag :: Deserializer :: new (& mut * self . de)) ? } } else { guard_recursion ! { self . de => seed . deserialize (& mut * self . de) ? } } ; self . had_comma = self . de . parser . comma () ? ; Ok (res) } else { Err (Error :: ExpectedMapColon) } } }
};
}
