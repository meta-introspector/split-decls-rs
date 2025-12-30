// Generated macro for impl_105 (impl)
macro_rules! Depcrate_de_textimpl_105 {
() => {
// Module: crate::de::text
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'de > TextDeserializer < 'de > { # [doc = " Returns a next string as concatenated content of consequent [`Text`] and"] # [doc = " [`CData`] events, used inside [`deserialize_primitives!()`]."] # [doc = ""] # [doc = " [`Text`]: crate::events::Event::Text"] # [doc = " [`CData`]: crate::events::Event::CData"] # [inline] fn read_string (self) -> Result < Cow < 'de , str > , DeError > { Ok (self . 0 . text) } }
};
}
