// Generated macro for read_dir (function)
macro_rules! Depcrate_dirread_dir {
() => {
// Module: crate::dir
// Provides: {"read_dir"}
// Dependencies: {}
# [doc = " Returns an iterator over the entries within a directory."] # [doc = ""] # [doc = " Wrapper for [`fs::read_dir`](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html)."] pub fn read_dir < P : Into < PathBuf > > (path : P) -> io :: Result < ReadDir > { let path = path . into () ; match fs :: read_dir (& path) { Ok (inner) => Ok (ReadDir { inner , path }) , Err (source) => Err (Error :: build (source , ErrorKind :: ReadDir , path)) , } }
};
}
