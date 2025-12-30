// Generated macro for OnDiskFile (struct)
macro_rules! Depcrate_store_impls_dynamic_typesOnDiskFile {
() => {
// Module: crate::store_impls::dynamic::types
// Provides: {"OnDiskFile"}
// Dependencies: {}
# [derive (Clone)] pub (crate) struct OnDiskFile < T : Clone > { # [doc = " The last known path of the file"] path : Arc < PathBuf > , # [doc = " the time the file was last modified"] mtime : SystemTime , state : OnDiskFileState < T > , }
};
}
