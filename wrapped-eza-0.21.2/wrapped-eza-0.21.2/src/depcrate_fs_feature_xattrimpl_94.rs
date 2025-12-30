// Generated macro for impl_94 (impl)
macro_rules! Depcrate_fs_feature_xattrimpl_94 {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a > io :: Write for BorrowedWriter < 'a > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . buffer . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . buffer . flush () } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . buffer . write_all (buf) } }
};
}
