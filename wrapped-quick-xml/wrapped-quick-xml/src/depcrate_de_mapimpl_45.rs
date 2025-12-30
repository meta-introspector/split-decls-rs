// Generated macro for impl_45 (impl)
macro_rules! Depcrate_de_mapimpl_45 {
() => {
// Module: crate::de::map
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'de , 'd , 'm , R , E > MapValueDeserializer < 'de , 'd , 'm , R , E > where R : XmlRead < 'de > , E : EntityResolver , { # [doc = " Returns a next string as concatenated content of consequent [`Text`] and"] # [doc = " [`CData`] events, used inside [`deserialize_primitives!()`]."] # [doc = ""] # [doc = " [`Text`]: crate::events::Event::Text"] # [doc = " [`CData`]: crate::events::Event::CData"] # [inline] fn read_string (& mut self) -> Result < Cow < 'de , str > , DeError > { self . map . de . read_string_impl (self . fixed_name) } }
};
}
