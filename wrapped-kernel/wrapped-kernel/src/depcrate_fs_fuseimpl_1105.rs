// Generated macro for impl_1105 (impl)
macro_rules! Depcrate_fs_fuseimpl_1105 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1105"}
// Dependencies: {}
# [async_trait] impl ObjectInterface for FuseFileHandle { async fn poll (& self , event : PollEvent) -> io :: Result < PollEvent > { self . 0 . lock () . await . poll (event) . await } async fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . lock () . await . read (buf) } async fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . 0 . lock () . await . write (buf) } async fn lseek (& self , offset : isize , whence : SeekWhence) -> io :: Result < isize > { self . 0 . lock () . await . lseek (offset , whence) } async fn fstat (& self) -> io :: Result < FileAttr > { self . 0 . lock () . await . fstat () } async fn truncate (& self , size : usize) -> io :: Result < () > { let attr = FileAttr { st_size : size . try_into () . unwrap () , .. FileAttr :: default () } ; self . 0 . lock () . await . set_attr (attr , SetAttrValidFields :: FATTR_SIZE) . map (| _ | ()) } async fn chmod (& self , access_permission : AccessPermission) -> io :: Result < () > { let attr = FileAttr { st_mode : access_permission , .. FileAttr :: default () } ; self . 0 . lock () . await . set_attr (attr , SetAttrValidFields :: FATTR_MODE) . map (| _ | ()) } }
};
}
