// Generated macro for create_dir (function)
macro_rules! Depcratecreate_dir {
() => {
// Module: crate
// Provides: {"create_dir"}
// Dependencies: {}
# [doc = " Creates a new, empty directory at the provided path."] # [doc = ""] # [doc = " Wrapper for [`fs::create_dir`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir.html)."] pub fn create_dir < P > (path : P) -> io :: Result < () > where P : AsRef < Path > , { let path = path . as_ref () ; fs :: create_dir (path) . map_err (| source | Error :: build (source , ErrorKind :: CreateDir , path)) }
};
}
