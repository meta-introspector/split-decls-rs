// Generated macro for source_dir (function)
macro_rules! Depcratesource_dir {
() => {
// Module: crate
// Provides: {"source_dir"}
// Dependencies: {}
pub fn source_dir () -> PathBuf { Path :: new (env ! ("CARGO_MANIFEST_DIR")) . join ("openssl") }
};
}
