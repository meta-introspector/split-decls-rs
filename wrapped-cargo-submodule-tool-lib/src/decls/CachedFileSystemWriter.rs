macro_rules! CachedFileSystemWriter {
    () => {
        # [cfg (not (feature = "git_enabled"))] pub struct CachedFileSystemWriter ;
    };
}

CachedFileSystemWriter!()