// Generated macro for impl_1176 (impl)
macro_rules! Depcrate_fs_uhyveimpl_1176 {
() => {
// Module: crate::fs::uhyve
// Provides: {"impl_1176"}
// Dependencies: {}
impl Read for UhyveFileHandleInner { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { let mut read_params = ReadParams { fd : self . 0 , buf : GuestVirtAddr :: new (buf . as_mut_ptr () as u64) , len : buf . len () , ret : 0 , } ; uhyve_hypercall (Hypercall :: FileRead (& mut read_params)) ; if read_params . ret >= 0 { Ok (read_params . ret . try_into () . unwrap ()) } else { Err (Errno :: Io) } } }
};
}
