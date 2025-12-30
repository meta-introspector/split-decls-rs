// Generated macro for impl_13 (impl)
macro_rules! Depcrate_de_attributesimpl_13 {
() => {
// Module: crate::de::attributes
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'de > IntoDeserializer < 'de , DeError > for Attributes < 'de > { type Deserializer = AttributesDeserializer < 'de > ; # [inline] fn into_deserializer (self) -> Self :: Deserializer { self . into_map_access ("") } }
};
}
