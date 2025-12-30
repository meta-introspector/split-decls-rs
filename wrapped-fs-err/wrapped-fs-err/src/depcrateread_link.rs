// Generated macro for read_link (function)
macro_rules! Depcrateread_link {
() => {
// Module: crate
// Provides: {"read_link"}
// Dependencies: {}
# [doc = " Reads a symbolic link, returning the file that the link points to."] # [doc = ""] # [doc = " Wrapper for [`fs::read_link`](https://doc.rust-lang.org/stable/std/fs/fn.read_link.html)."] pub fn read_link < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { let path = path . as_ref () ; fs :: read_link (path) . map_err (| source | Error :: build (source , ErrorKind :: ReadLink , path)) }
};
}
