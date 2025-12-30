// Generated macro for open (function)
macro_rules! Depcrate_fileopen {
() => {
// Module: crate::file
// Provides: {"open"}
// Dependencies: {}
pub (crate) fn open (path : & Path) -> Result < std :: fs :: File , impl FnOnce (PathBuf) -> io :: Error > { fs :: File :: open (path) . map_err (| err | | path | Error :: build (err , ErrorKind :: OpenFile , path)) }
};
}
