// Generated macro for remove_dir_all (function)
macro_rules! Depcrateremove_dir_all {
() => {
// Module: crate
// Provides: {"remove_dir_all"}
// Dependencies: {}
# [doc = " Removes a directory at this path, after removing all its contents. Use carefully!"] # [doc = ""] # [doc = " Wrapper for [`fs::remove_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir_all.html)."] pub fn remove_dir_all < P > (path : P) -> io :: Result < () > where P : AsRef < Path > , { let path = path . as_ref () ; fs :: remove_dir_all (path) . map_err (| source | Error :: build (source , ErrorKind :: RemoveDir , path)) }
};
}
