// Generated macro for open_file (function)
macro_rules! Depcrateopen_file {
() => {
// Module: crate
// Provides: {"open_file"}
// Dependencies: {}
# [cfg (target_family = "windows")] fn open_file < P : AsRef < Path > > (path : P) -> io :: Result < fs :: File > { use std :: { fs :: OpenOptions , os :: windows :: fs :: OpenOptionsExt } ; use windows_sys :: Win32 :: Storage :: FileSystem :: FILE_FLAG_BACKUP_SEMANTICS ; OpenOptions :: new () . access_mode (0) . custom_flags (FILE_FLAG_BACKUP_SEMANTICS) . open (path) }
};
}
