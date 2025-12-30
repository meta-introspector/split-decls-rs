// Generated macro for rename (function)
macro_rules! Depcraterename {
() => {
// Module: crate
// Provides: {"rename"}
// Dependencies: {}
# [doc = " Rename a file or directory to a new name, replacing the original file if to already exists."] # [doc = ""] # [doc = " Wrapper for [`fs::rename`](https://doc.rust-lang.org/stable/std/fs/fn.rename.html)."] pub fn rename < P : AsRef < Path > , Q : AsRef < Path > > (from : P , to : Q) -> io :: Result < () > { let from = from . as_ref () ; let to = to . as_ref () ; fs :: rename (from , to) . map_err (| source | SourceDestError :: build (source , SourceDestErrorKind :: Rename , from , to)) }
};
}
