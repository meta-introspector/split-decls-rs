// Generated macro for is_bsd_like (function)
macro_rules! Depcrate_archive_writeris_bsd_like {
() => {
// Module: crate::archive_writer
// Provides: {"is_bsd_like"}
// Dependencies: {}
fn is_bsd_like (kind : ArchiveKind) -> bool { match kind { ArchiveKind :: Gnu | ArchiveKind :: Gnu64 | ArchiveKind :: AixBig | ArchiveKind :: Coff => false , ArchiveKind :: Bsd | ArchiveKind :: Darwin | ArchiveKind :: Darwin64 => true , } }
};
}
