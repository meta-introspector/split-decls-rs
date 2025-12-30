// Generated macro for format_from_ext (function)
macro_rules! Depcrate_repository_archiveformat_from_ext {
() => {
// Module: crate::repository::archive
// Provides: {"format_from_ext"}
// Dependencies: {}
fn format_from_ext (path : & Path) -> anyhow :: Result < archive :: Format > { Ok (match path . extension () . and_then (std :: ffi :: OsStr :: to_str) { None => bail ! ("Cannot derive archive format from a file without extension") , Some ("tar") => archive :: Format :: Tar , Some ("gz") => archive :: Format :: TarGz { compression_level : None , } , Some ("zip") => archive :: Format :: Zip { compression_level : None , } , Some ("stream") => archive :: Format :: InternalTransientNonPersistable , Some (ext) => bail ! ("Format for extension '{ext}' is unsupported") , }) }
};
}
