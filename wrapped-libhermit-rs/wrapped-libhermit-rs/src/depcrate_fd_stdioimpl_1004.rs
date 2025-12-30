// Generated macro for impl_1004 (impl)
macro_rules! Depcrate_fd_stdioimpl_1004 {
() => {
// Module: crate::fd::stdio
// Provides: {"impl_1004"}
// Dependencies: {}
# [async_trait] impl ObjectInterface for UhyveStdin { async fn isatty (& self) -> io :: Result < bool > { Ok (true) } async fn fstat (& self) -> io :: Result < FileAttr > { let attr = FileAttr { st_mode : AccessPermission :: S_IFCHR , .. Default :: default () } ; Ok (attr) } }
};
}
