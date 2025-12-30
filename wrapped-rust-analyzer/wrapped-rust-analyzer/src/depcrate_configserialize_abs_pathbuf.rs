// Generated macro for serialize_abs_pathbuf (function)
macro_rules! Depcrate_configserialize_abs_pathbuf {
() => {
// Module: crate::config
// Provides: {"serialize_abs_pathbuf"}
// Dependencies: {}
fn serialize_abs_pathbuf < S > (path : & AbsPathBuf , se : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { let path : & Utf8Path = path . as_ref () ; se . serialize_str (path . as_str ()) }
};
}
