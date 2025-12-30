// Generated macro for ReadDir (struct)
macro_rules! Depcrate_dirReadDir {
() => {
// Module: crate::dir
// Provides: {"ReadDir"}
// Dependencies: {}
# [doc = " Wrapper around [`std::fs::ReadDir`][std::fs::ReadDir] which adds more"] # [doc = " helpful information to all errors."] # [doc = ""] # [doc = " This struct is created via [`fs_err::read_dir`][fs_err::read_dir]."] # [doc = ""] # [doc = " [std::fs::ReadDir]: https://doc.rust-lang.org/stable/std/fs/struct.ReadDir.html"] # [doc = " [fs_err::read_dir]: fn.read_dir.html"] # [derive (Debug)] pub struct ReadDir { inner : fs :: ReadDir , path : PathBuf , }
};
}
