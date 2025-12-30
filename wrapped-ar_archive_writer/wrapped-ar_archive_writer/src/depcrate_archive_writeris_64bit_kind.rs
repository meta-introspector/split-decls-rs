// Generated macro for is_64bit_kind (function)
macro_rules! Depcrate_archive_writeris_64bit_kind {
() => {
// Module: crate::archive_writer
// Provides: {"is_64bit_kind"}
// Dependencies: {}
fn is_64bit_kind (kind : ArchiveKind) -> bool { match kind { ArchiveKind :: Gnu | ArchiveKind :: Bsd | ArchiveKind :: Darwin | ArchiveKind :: Coff => false , ArchiveKind :: AixBig | ArchiveKind :: Darwin64 | ArchiveKind :: Gnu64 => true , } }
};
}
