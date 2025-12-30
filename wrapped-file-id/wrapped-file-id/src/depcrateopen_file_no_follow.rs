// Generated macro for open_file_no_follow (function)
macro_rules! Depcrateopen_file_no_follow {
() => {
// Module: crate
// Provides: {"open_file_no_follow"}
// Dependencies: {}
# [cfg (target_family = "windows")] fn open_file_no_follow < P : AsRef < Path > > (path : P) -> io :: Result < fs :: File > { use std :: { fs :: OpenOptions , os :: windows :: fs :: OpenOptionsExt } ; use windows_sys :: Win32 :: Storage :: FileSystem :: { FILE_FLAG_BACKUP_SEMANTICS , FILE_FLAG_OPEN_REPARSE_POINT , } ; OpenOptions :: new () . access_mode (0) . custom_flags (FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT) . open (path) }
};
}
