// Generated macro for impl_1181 (impl)
macro_rules! Depcrate_fs_uhyveimpl_1181 {
() => {
// Module: crate::fs::uhyve
// Provides: {"impl_1181"}
// Dependencies: {}
# [async_trait] impl ObjectInterface for UhyveFileHandle { async fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . lock () . await . read (buf) } async fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . 0 . lock () . await . write (buf) } async fn lseek (& self , offset : isize , whence : SeekWhence) -> io :: Result < isize > { self . 0 . lock () . await . lseek (offset , whence) } }
};
}
