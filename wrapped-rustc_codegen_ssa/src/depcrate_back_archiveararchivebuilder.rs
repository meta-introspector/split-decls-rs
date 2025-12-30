// Generated macro for ArArchiveBuilder (struct)
macro_rules! Depcrate_back_archiveArArchiveBuilder {
() => {
// Module: crate::back::archive
// Provides: {"ArArchiveBuilder"}
// Dependencies: {}
# [must_use = "must call build() to finish building the archive"] pub struct ArArchiveBuilder < 'a > { sess : & 'a Session , object_reader : & 'static ObjectReader , src_archives : Vec < (PathBuf , Mmap) > , entries : Vec < (Vec < u8 > , ArchiveEntry) > , }
};
}
