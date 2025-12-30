// Generated macro for exists (function)
macro_rules! Depcrateexists {
() => {
// Module: crate
// Provides: {"exists"}
// Dependencies: {}
# [doc = " Returns `Ok(true)` if the path points at an existing entity."] # [doc = ""] # [doc = " Wrapper for [`fs::exists`](https://doc.rust-lang.org/stable/std/fs/fn.exists.html)."] # [cfg (rustc_1_81)] pub fn exists < P : AsRef < Path > > (path : P) -> io :: Result < bool > { let path = path . as_ref () ; fs :: exists (path) . map_err (| source | Error :: build (source , ErrorKind :: FileExists , path)) }
};
}
