// Generated macro for impl_1178 (impl)
macro_rules! Depcrate_fs_uhyveimpl_1178 {
() => {
// Module: crate::fs::uhyve
// Provides: {"impl_1178"}
// Dependencies: {}
impl Drop for UhyveFileHandleInner { fn drop (& mut self) { let mut close_params = CloseParams { fd : self . 0 , ret : 0 } ; uhyve_hypercall (Hypercall :: FileClose (& mut close_params)) ; if close_params . ret != 0 { let ret = close_params . ret ; panic ! ("Can't close fd {} - return value {ret}" , self . 0) ; } } }
};
}
