// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl SerializationFormat { pub fn from_file_extension (extension : & str) -> Self { match extension { "qlog" => SerializationFormat :: QlogJson , "sqlog" => SerializationFormat :: QlogJsonSeq , "json" => Self :: NetlogJson , _ => SerializationFormat :: Unknown , } } }
};
}
