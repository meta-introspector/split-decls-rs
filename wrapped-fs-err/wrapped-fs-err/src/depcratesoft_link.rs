// Generated macro for soft_link (function)
macro_rules! Depcratesoft_link {
() => {
// Module: crate
// Provides: {"soft_link"}
// Dependencies: {}
# [doc = " Wrapper for [`fs::soft_link`](https://doc.rust-lang.org/stable/std/fs/fn.soft_link.html)."] # [deprecated = "replaced with std::os::unix::fs::symlink and \
std::os::windows::fs::{symlink_file, symlink_dir}"] pub fn soft_link < P : AsRef < Path > , Q : AsRef < Path > > (src : P , dst : Q) -> io :: Result < () > { let src = src . as_ref () ; let dst = dst . as_ref () ; # [allow (deprecated)] fs :: soft_link (src , dst) . map_err (| source | SourceDestError :: build (source , SourceDestErrorKind :: SoftLink , src , dst)) }
};
}
