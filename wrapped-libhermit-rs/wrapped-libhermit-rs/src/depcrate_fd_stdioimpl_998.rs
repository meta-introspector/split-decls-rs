// Generated macro for impl_998 (impl)
macro_rules! Depcrate_fd_stdioimpl_998 {
() => {
// Module: crate::fd::stdio
// Provides: {"impl_998"}
// Dependencies: {}
# [async_trait] impl ObjectInterface for GenericStdout { async fn poll (& self , event : PollEvent) -> io :: Result < PollEvent > { let available = PollEvent :: POLLOUT | PollEvent :: POLLWRNORM | PollEvent :: POLLWRBAND ; Ok (event & available) } async fn write (& self , buf : & [u8]) -> io :: Result < usize > { CONSOLE . lock () . write (buf) } async fn isatty (& self) -> io :: Result < bool > { Ok (true) } async fn fstat (& self) -> io :: Result < FileAttr > { let attr = FileAttr { st_mode : AccessPermission :: S_IFCHR , .. Default :: default () } ; Ok (attr) } }
};
}
