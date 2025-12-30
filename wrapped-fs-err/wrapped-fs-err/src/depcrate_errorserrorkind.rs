// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorsErrorKind {
() => {
// Module: crate::errors
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] pub (crate) enum ErrorKind { OpenFile , CreateFile , CreateDir , SyncFile , SetLen , Metadata , Clone , SetPermissions , Read , Seek , Write , Flush , ReadDir , RemoveFile , RemoveDir , Canonicalize , ReadLink , SymlinkMetadata , # [allow (dead_code)] FileExists , Lock , Unlock , # [cfg (windows)] SeekRead , # [cfg (windows)] SeekWrite , # [cfg (unix)] ReadAt , # [cfg (unix)] WriteAt , }
};
}
