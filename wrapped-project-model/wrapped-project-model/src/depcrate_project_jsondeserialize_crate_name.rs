// Generated macro for deserialize_crate_name (function)
macro_rules! Depcrate_project_jsondeserialize_crate_name {
() => {
// Module: crate::project_json
// Provides: {"deserialize_crate_name"}
// Dependencies: {}
fn deserialize_crate_name < 'de , D > (de : D) -> std :: result :: Result < CrateName , D :: Error > where D : de :: Deserializer < 'de > , { let name = String :: deserialize (de) ? ; CrateName :: new (& name) . map_err (| err | de :: Error :: custom (format ! ("invalid crate name: {err:?}"))) }
};
}
