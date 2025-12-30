// Generated macro for is_darwin (function)
macro_rules! Depcrate_archive_writeris_darwin {
() => {
// Module: crate::archive_writer
// Provides: {"is_darwin"}
// Dependencies: {}
fn is_darwin (kind : ArchiveKind) -> bool { matches ! (kind , ArchiveKind :: Darwin | ArchiveKind :: Darwin64) }
};
}
