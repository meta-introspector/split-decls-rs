// Generated macro for Deserializer (struct)
macro_rules! Depcrate_deDeserializer {
() => {
// Module: crate::de
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " The RON deserializer."] # [doc = ""] # [doc = " If you just want to simply deserialize a value,"] # [doc = " you can use the [`from_str`] convenience function."] pub struct Deserializer < 'de > { pub (crate) parser : Parser < 'de > , newtype_variant : bool , serde_content_newtype : bool , last_identifier : Option < & 'de str > , recursion_limit : Option < usize > , }
};
}
