// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Error kinds"] # [derive (Debug)] pub enum ErrorKind { # [doc = " Generic error"] # [doc = ""] # [doc = " May be used in cases where a platform specific error is mapped to this type, or for opaque"] # [doc = " internal errors."] Generic (String) , # [doc = " I/O errors."] Io (io :: Error) , # [doc = " A path does not exist."] PathNotFound , # [doc = " Attempted to remove a watch that does not exist."] WatchNotFound , # [doc = " An invalid value was passed as runtime configuration."] InvalidConfig (Config) , # [doc = " Can't watch (more) files, limit on the total number of inotify watches reached"] MaxFilesWatch , }
};
}
