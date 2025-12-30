// Generated macro for impl_256 (impl)
macro_rules! Depcrate_schemaimpl_256 {
() => {
// Module: crate::schema
// Provides: {"impl_256"}
// Dependencies: {}
impl JsonSchema for TomlValueWrapper { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "TomlValue" . into () } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { generator . subschema_for :: < serde_json :: Value > () . into () } }
};
}
