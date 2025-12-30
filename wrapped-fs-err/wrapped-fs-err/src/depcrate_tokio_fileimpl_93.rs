// Generated macro for impl_93 (impl)
macro_rules! Depcrate_tokio_fileimpl_93 {
() => {
// Module: crate::tokio::file
// Provides: {"impl_93"}
// Dependencies: {}
# [doc = " Methods added by fs-err that are not available on"] # [doc = " [`tokio::fs::File`]."] impl File { # [doc = " Creates a [`File`](struct.File.html) from a raw file and its path."] pub fn from_parts < P > (file : TokioFile , path : P) -> Self where P : Into < PathBuf > , { File { tokio : file , path : path . into () , } } # [doc = " Extract the raw file and its path from this [`File`](struct.File.html)."] pub fn into_parts (self) -> (TokioFile , PathBuf) { (self . tokio , self . path) } # [doc = " Returns a reference to the underlying [`tokio::fs::File`]."] pub fn file (& self) -> & TokioFile { & self . tokio } # [doc = " Returns a mutable reference to the underlying [`tokio::fs::File`]."] pub fn file_mut (& mut self) -> & mut TokioFile { & mut self . tokio } # [doc = " Returns a reference to the path that this file was created with."] pub fn path (& self) -> & Path { & self . path } # [doc = " Wrap the error in information specific to this `File` object."] fn error (& self , source : io :: Error , kind : ErrorKind) -> io :: Error { Error :: build (source , kind , & self . path) } }
};
}
