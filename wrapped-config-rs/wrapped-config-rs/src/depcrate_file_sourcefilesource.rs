// Generated macro for FileSource (trait)
macro_rules! Depcrate_file_sourceFileSource {
() => {
// Module: crate::file::source
// Provides: {"FileSource"}
// Dependencies: {}
# [doc = " Describes where the [`File`][super::File] is sourced"] pub trait FileSource < T > : Debug + Clone where T : Format + FileStoredFormat , { fn resolve (& self , format_hint : Option < T > ,) -> Result < FileSourceResult , Box < dyn Error + Send + Sync > > ; }
};
}
