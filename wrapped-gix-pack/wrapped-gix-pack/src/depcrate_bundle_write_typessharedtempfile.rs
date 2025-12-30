// Generated macro for SharedTempFile (type)
macro_rules! Depcrate_bundle_write_typesSharedTempFile {
() => {
// Module: crate::bundle::write::types
// Provides: {"SharedTempFile"}
// Dependencies: {}
pub (crate) type SharedTempFile = Arc < parking_lot :: Mutex < std :: io :: BufWriter < gix_tempfile :: Handle < Writable > > > > ;
};
}
