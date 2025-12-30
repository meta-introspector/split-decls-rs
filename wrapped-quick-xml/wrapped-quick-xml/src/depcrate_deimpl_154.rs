// Generated macro for impl_154 (impl)
macro_rules! Depcrate_deimpl_154 {
() => {
// Module: crate::de
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'de , R , E > IntoDeserializer < 'de , DeError > for & mut Deserializer < 'de , R , E > where R : XmlRead < 'de > , E : EntityResolver , { type Deserializer = Self ; # [inline] fn into_deserializer (self) -> Self { self } }
};
}
