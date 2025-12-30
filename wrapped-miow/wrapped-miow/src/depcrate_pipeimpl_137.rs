// Generated macro for impl_137 (impl)
macro_rules! Depcrate_pipeimpl_137 {
() => {
// Module: crate::pipe
// Provides: {"impl_137"}
// Dependencies: {}
impl Read for & NamedPipe { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { with_threadlocal_overlapped (| overlapped | unsafe { self . 0 . read_overlapped_wait (buf , overlapped . raw ()) }) } }
};
}
