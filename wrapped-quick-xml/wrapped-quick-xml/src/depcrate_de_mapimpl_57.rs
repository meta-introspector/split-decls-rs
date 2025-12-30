// Generated macro for impl_57 (impl)
macro_rules! Depcrate_de_mapimpl_57 {
() => {
// Module: crate::de::map
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'de , 'd , R , E > ElementDeserializer < 'de , 'd , R , E > where R : XmlRead < 'de > , E : EntityResolver , { # [doc = " Returns a next string as concatenated content of consequent [`Text`] and"] # [doc = " [`CData`] events, used inside [`deserialize_primitives!()`]."] # [doc = ""] # [doc = " [`Text`]: crate::events::Event::Text"] # [doc = " [`CData`]: crate::events::Event::CData"] # [inline] fn read_string (& mut self) -> Result < Cow < 'de , str > , DeError > { self . de . read_text (self . start . name ()) } }
};
}
