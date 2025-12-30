// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_fd_stdioimpl_1007 {
() => {
// Module: crate::fd::stdio
// Provides: {"impl_1007"}
// Dependencies: {}
# [async_trait] impl ObjectInterface for UhyveStdout { async fn poll (& self , event : PollEvent) -> io :: Result < PollEvent > { let available = PollEvent :: POLLOUT | PollEvent :: POLLWRNORM | PollEvent :: POLLWRBAND ; Ok (event & available) } async fn write (& self , buf : & [u8]) -> io :: Result < usize > { let write_params = WriteParams { fd : STDOUT_FILENO , buf : GuestVirtAddr :: new (buf . as_ptr () as u64) , len : buf . len () , } ; uhyve_hypercall (Hypercall :: FileWrite (& write_params)) ; Ok (write_params . len) } async fn isatty (& self) -> io :: Result < bool > { Ok (true) } async fn fstat (& self) -> io :: Result < FileAttr > { let attr = FileAttr { st_mode : AccessPermission :: S_IFCHR , .. Default :: default () } ; Ok (attr) } }
};
}
