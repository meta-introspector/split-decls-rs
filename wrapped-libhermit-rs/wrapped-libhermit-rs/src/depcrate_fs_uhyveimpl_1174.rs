// Generated macro for impl_1174 (impl)
macro_rules! Depcrate_fs_uhyveimpl_1174 {
() => {
// Module: crate::fs::uhyve
// Provides: {"impl_1174"}
// Dependencies: {}
impl UhyveFileHandleInner { pub fn new (fd : i32) -> Self { Self (fd) } fn lseek (& self , offset : isize , whence : SeekWhence) -> io :: Result < isize > { let mut lseek_params = LseekParams { fd : self . 0 , offset , whence : u8 :: from (whence) . into () , } ; uhyve_hypercall (Hypercall :: FileLseek (& mut lseek_params)) ; if lseek_params . offset >= 0 { Ok (lseek_params . offset) } else { Err (Errno :: Inval) } } }
};
}
