// Generated macro for remove_dir (function)
macro_rules! Depcrateremove_dir {
() => {
// Module: crate
// Provides: {"remove_dir"}
// Dependencies: {}
# [doc = " Removes an empty directory."] # [doc = ""] # [doc = " Wrapper for [`fs::remove_dir`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir.html)."] pub fn remove_dir < P > (path : P) -> io :: Result < () > where P : AsRef < Path > , { let path = path . as_ref () ; fs :: remove_dir (path) . map_err (| source | Error :: build (source , ErrorKind :: RemoveDir , path)) }
};
}
