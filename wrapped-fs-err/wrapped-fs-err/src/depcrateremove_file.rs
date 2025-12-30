// Generated macro for remove_file (function)
macro_rules! Depcrateremove_file {
() => {
// Module: crate
// Provides: {"remove_file"}
// Dependencies: {}
# [doc = " Removes a file from the filesystem."] # [doc = ""] # [doc = " Wrapper for [`fs::remove_file`](https://doc.rust-lang.org/stable/std/fs/fn.remove_file.html)."] pub fn remove_file < P > (path : P) -> io :: Result < () > where P : AsRef < Path > , { let path = path . as_ref () ; fs :: remove_file (path) . map_err (| source | Error :: build (source , ErrorKind :: RemoveFile , path)) }
};
}
