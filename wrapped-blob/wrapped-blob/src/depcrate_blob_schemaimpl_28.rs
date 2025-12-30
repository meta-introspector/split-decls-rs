// Generated macro for impl_28 (impl)
macro_rules! Depcrate_blob_schemaimpl_28 {
() => {
// Module: crate::blob_schema
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'de > serde :: Deserialize < 'de > for NeverSchema { fn deserialize < D > (_ : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: de :: Error ; Err (D :: Error :: custom ("Attempted to read 1.0 blob format from ICU4X 2.0: please run ICU4X 2.0 datagen to generate a new file.")) } }
};
}
