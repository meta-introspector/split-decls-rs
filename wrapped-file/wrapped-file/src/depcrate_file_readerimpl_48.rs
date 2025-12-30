// Generated macro for impl_48 (impl)
macro_rules! Depcrate_file_readerimpl_48 {
() => {
// Module: crate::file_reader
// Provides: {"impl_48"}
// Dependencies: {}
impl std :: fmt :: Display for FileReadError { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { FileReadError :: AbortedEarly => write ! (f , "FileReader aborted early") , FileReadError :: NotFound (msg) => write ! (f , "FileReader cannot find blob: {msg}") , FileReadError :: NotReadable (msg) => { write ! (f , "FileReader cannot read contents of blob: {msg}") } FileReadError :: Security (msg) => { write ! (f , "FileReader encountered a security exception: {msg}") } } } }
};
}
