// Generated macro for impl_27 (impl)
macro_rules! Depcrate_commitimpl_27 {
() => {
// Module: crate::commit
// Provides: {"impl_27"}
// Dependencies: {}
impl File { # [doc = " Commit the changes written to this lock file and overwrite the original file atomically, returning the resource path"] # [doc = " and an open file handle on success."] pub fn commit (mut self) -> Result < (PathBuf , Option < std :: fs :: File >) , Error < Self > > { let resource_path = self . resource_path () ; match self . inner . persist (& resource_path) { Ok (possibly_file) => Ok ((resource_path , possibly_file)) , Err (err) => Err (Error { error : err . error , instance : { self . inner = err . handle ; self } , }) , } } }
};
}
