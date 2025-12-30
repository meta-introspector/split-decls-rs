// Generated macro for BackedUpFile (struct)
macro_rules! Depcrate_boltBackedUpFile {
() => {
// Module: crate::bolt
// Provides: {"BackedUpFile"}
// Dependencies: {}
# [doc = " Copies a file to a temporary location and restores it (copies it back) when it is dropped."] pub struct BackedUpFile { original : Utf8PathBuf , backup : tempfile :: TempPath , }
};
}
