// Generated macro for copy (function)
macro_rules! Depcratecopy {
() => {
// Module: crate
// Provides: {"copy"}
// Dependencies: {}
# [doc = " Copies the contents of one file to another. This function will also copy the"] # [doc = " permission bits of the original file to the destination file."] # [doc = ""] # [doc = " Wrapper for [`fs::copy`](https://doc.rust-lang.org/stable/std/fs/fn.copy.html)."] pub fn copy < P , Q > (from : P , to : Q) -> io :: Result < u64 > where P : AsRef < Path > , Q : AsRef < Path > , { let from = from . as_ref () ; let to = to . as_ref () ; fs :: copy (from , to) . map_err (| source | SourceDestError :: build (source , SourceDestErrorKind :: Copy , from , to)) }
};
}
