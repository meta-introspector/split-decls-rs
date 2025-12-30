// Generated macro for FileIdCache (trait)
macro_rules! Depcrate_cacheFileIdCache {
() => {
// Module: crate::cache
// Provides: {"FileIdCache"}
// Dependencies: {}
# [doc = " The interface of a file ID cache."] # [doc = ""] # [doc = " This trait can be implemented for an existing cache, if it already holds `FileId`s."] pub trait FileIdCache { # [doc = " Get a `FileId` from the cache for a given `path`."] # [doc = ""] # [doc = " If the path is not cached, `None` should be returned and there should not be any attempt to read the file ID from disk."] fn cached_file_id (& self , path : & Path) -> Option < impl AsRef < FileId > > ; # [doc = " Add a new path to the cache or update its value."] # [doc = ""] # [doc = " This will be called if a new file or directory is created or if an existing file is overridden."] fn add_path (& mut self , path : & Path , recursive_mode : RecursiveMode) ; # [doc = " Remove a path from the cache."] # [doc = ""] # [doc = " This will be called if a file or directory is deleted."] fn remove_path (& mut self , path : & Path) ; # [doc = " Re-scan all `root_paths`."] # [doc = ""] # [doc = " This will be called if the notification back-end has dropped events."] # [doc = " The root paths are passed as argument, so the implementer doesn't have to store them."] # [doc = ""] # [doc = " The default implementation calls `add_path` for each root path."] fn rescan (& mut self , root_paths : & [(PathBuf , RecursiveMode)]) { for (path , recursive_mode) in root_paths { self . add_path (path , * recursive_mode) ; } } }
};
}
