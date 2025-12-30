// Generated macro for impl_138 (impl)
macro_rules! Depcrate_pipeimpl_138 {
() => {
// Module: crate::pipe
// Provides: {"impl_138"}
// Dependencies: {}
impl Write for NamedPipe { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { with_threadlocal_overlapped (| overlapped | unsafe { self . 0 . write_overlapped_wait (buf , overlapped . raw ()) }) } fn flush (& mut self) -> io :: Result < () > { < & NamedPipe as Write > :: flush (& mut & * self) } }
};
}
