// Generated macro for hard_link (function)
macro_rules! Depcratehard_link {
() => {
// Module: crate
// Provides: {"hard_link"}
// Dependencies: {}
# [doc = " Creates a new hard link on the filesystem."] # [doc = ""] # [doc = " Wrapper for [`fs::hard_link`](https://doc.rust-lang.org/stable/std/fs/fn.hard_link.html)."] pub fn hard_link < P : AsRef < Path > , Q : AsRef < Path > > (src : P , dst : Q) -> io :: Result < () > { let src = src . as_ref () ; let dst = dst . as_ref () ; fs :: hard_link (src , dst) . map_err (| source | SourceDestError :: build (source , SourceDestErrorKind :: HardLink , src , dst)) }
};
}
