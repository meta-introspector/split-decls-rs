// Generated macro for DIRECTORY_SIZE_CACHE (static)
macro_rules! Depcrate_fs_fileDIRECTORY_SIZE_CACHE {
() => {
// Module: crate::fs::file
// Provides: {"DIRECTORY_SIZE_CACHE"}
// Dependencies: {}
# [allow (clippy :: type_complexity)] # [cfg (unix)] static DIRECTORY_SIZE_CACHE : LazyLock < Mutex < HashMap < (u64 , u64) , (u64 , u64) > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;
};
}
