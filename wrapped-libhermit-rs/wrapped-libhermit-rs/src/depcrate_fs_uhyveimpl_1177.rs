// Generated macro for impl_1177 (impl)
macro_rules! Depcrate_fs_uhyveimpl_1177 {
() => {
// Module: crate::fs::uhyve
// Provides: {"impl_1177"}
// Dependencies: {}
impl Write for UhyveFileHandleInner { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { let write_params = WriteParams { fd : self . 0 , buf : GuestVirtAddr :: new (buf . as_ptr () as u64) , len : buf . len () , } ; uhyve_hypercall (Hypercall :: FileWrite (& write_params)) ; Ok (write_params . len) } fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
