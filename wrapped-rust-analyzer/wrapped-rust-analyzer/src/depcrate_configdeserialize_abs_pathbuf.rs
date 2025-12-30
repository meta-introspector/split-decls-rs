// Generated macro for deserialize_abs_pathbuf (function)
macro_rules! Depcrate_configdeserialize_abs_pathbuf {
() => {
// Module: crate::config
// Provides: {"deserialize_abs_pathbuf"}
// Dependencies: {}
fn deserialize_abs_pathbuf < 'de , D > (de : D) -> std :: result :: Result < AbsPathBuf , D :: Error > where D : serde :: de :: Deserializer < 'de > , { let path = String :: deserialize (de) ? ; AbsPathBuf :: try_from (path . as_ref ()) . map_err (| err | serde :: de :: Error :: custom (format ! ("invalid path name: {err:?}"))) }
};
}
