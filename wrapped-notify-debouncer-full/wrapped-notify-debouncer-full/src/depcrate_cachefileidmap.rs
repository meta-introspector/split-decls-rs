// Generated macro for FileIdMap (struct)
macro_rules! Depcrate_cacheFileIdMap {
() => {
// Module: crate::cache
// Provides: {"FileIdMap"}
// Dependencies: {}
# [doc = " A cache to hold the file system IDs of all watched files."] # [doc = ""] # [doc = " The file ID cache uses unique file IDs provided by the file system and is used to stitch together"] # [doc = " rename events in case the notification back-end doesn't emit rename cookies."] # [derive (Debug , Clone , Default)] pub struct FileIdMap { paths : HashMap < PathBuf , FileId > , }
};
}
