// Generated macro for create_dir_all (function)
macro_rules! Depcratecreate_dir_all {
() => {
// Module: crate
// Provides: {"create_dir_all"}
// Dependencies: {}
# [doc = " Recursively create a directory and all of its parent components if they are missing."] # [doc = ""] # [doc = " Wrapper for [`fs::create_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir_all.html)."] pub fn create_dir_all < P > (path : P) -> io :: Result < () > where P : AsRef < Path > , { let path = path . as_ref () ; fs :: create_dir_all (path) . map_err (| source | Error :: build (source , ErrorKind :: CreateDir , path)) }
};
}
