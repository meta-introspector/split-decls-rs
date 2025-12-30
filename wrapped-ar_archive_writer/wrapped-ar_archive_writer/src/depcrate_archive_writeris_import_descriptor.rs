// Generated macro for is_import_descriptor (function)
macro_rules! Depcrate_archive_writeris_import_descriptor {
() => {
// Module: crate::archive_writer
// Provides: {"is_import_descriptor"}
// Dependencies: {}
fn is_import_descriptor (name : & [u8]) -> bool { name . starts_with (coff_import_file :: IMPORT_DESCRIPTOR_PREFIX) || name . starts_with (coff_import_file :: NULL_IMPORT_DESCRIPTOR_SYMBOL_NAME) || (name . starts_with (coff_import_file :: NULL_THUNK_DATA_PREFIX) && name . ends_with (coff_import_file :: NULL_THUNK_DATA_SUFFIX)) }
};
}
