// Generated macro for FileStoredFormat (trait)
macro_rules! Depcrate_fileFileStoredFormat {
() => {
// Module: crate::file
// Provides: {"FileStoredFormat"}
// Dependencies: {}
# [doc = " An extension of [`Format`] trait."] # [doc = ""] # [doc = " Associates format with file extensions, therefore linking storage-agnostic notion of format to a file system."] pub trait FileStoredFormat : Format { # [doc = " Returns a vector of file extensions, for instance `[yml, yaml]`."] fn file_extensions (& self) -> & 'static [& 'static str] ; }
};
}
