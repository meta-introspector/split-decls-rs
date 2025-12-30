// Generated macro for create (function)
macro_rules! Depcrate_filecreate {
() => {
// Module: crate::file
// Provides: {"create"}
// Dependencies: {}
pub (crate) fn create (path : & Path) -> Result < std :: fs :: File , impl FnOnce (PathBuf) -> io :: Error > { fs :: File :: create (path) . map_err (| err | | path | Error :: build (err , ErrorKind :: CreateFile , path)) }
};
}
