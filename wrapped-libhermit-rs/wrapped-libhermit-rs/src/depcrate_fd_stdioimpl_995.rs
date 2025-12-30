// Generated macro for impl_995 (impl)
macro_rules! Depcrate_fd_stdioimpl_995 {
() => {
// Module: crate::fd::stdio
// Provides: {"impl_995"}
// Dependencies: {}
# [async_trait] impl ObjectInterface for GenericStdin { async fn poll (& self , event : PollEvent) -> io :: Result < PollEvent > { let available = if CONSOLE . lock () . read_ready () ? { PollEvent :: POLLIN | PollEvent :: POLLRDNORM | PollEvent :: POLLRDBAND } else { PollEvent :: empty () } ; Ok (event & available) } async fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { future :: poll_fn (| cx | { let read_bytes = CONSOLE . lock () . read (buf) ? ; if read_bytes > 0 { CONSOLE . lock () . write_all (& buf [.. read_bytes]) ? ; CONSOLE . lock () . flush () ? ; Poll :: Ready (Ok (read_bytes)) } else { CONSOLE_WAKER . lock () . register (cx . waker ()) ; Poll :: Pending } }) . await } async fn isatty (& self) -> io :: Result < bool > { Ok (true) } async fn fstat (& self) -> io :: Result < FileAttr > { let attr = FileAttr { st_mode : AccessPermission :: S_IFCHR , .. Default :: default () } ; Ok (attr) } }
};
}
