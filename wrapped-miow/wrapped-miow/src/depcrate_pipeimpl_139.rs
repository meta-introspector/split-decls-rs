// Generated macro for impl_139 (impl)
macro_rules! Depcrate_pipeimpl_139 {
() => {
// Module: crate::pipe
// Provides: {"impl_139"}
// Dependencies: {}
impl Write for & NamedPipe { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { with_threadlocal_overlapped (| overlapped | unsafe { self . 0 . write_overlapped_wait (buf , overlapped . raw ()) }) } fn flush (& mut self) -> io :: Result < () > { crate :: cvt (unsafe { FlushFileBuffers (self . 0 . raw ()) }) . map (| _ | ()) } }
};
}
