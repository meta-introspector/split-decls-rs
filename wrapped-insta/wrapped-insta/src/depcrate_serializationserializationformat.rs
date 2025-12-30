// Generated macro for SerializationFormat (enum)
macro_rules! Depcrate_serializationSerializationFormat {
() => {
// Module: crate::serialization
// Provides: {"SerializationFormat"}
// Dependencies: {}
pub enum SerializationFormat { # [cfg (feature = "csv")] Csv , # [cfg (feature = "ron")] Ron , # [cfg (feature = "toml")] Toml , Yaml , Json , JsonCompact , }
};
}
