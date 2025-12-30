// Generated macro for impl_40 (impl)
macro_rules! Depcrate_writer_bufferimpl_40 {
() => {
// Module: crate::writer::buffer
// Provides: {"impl_40"}
// Dependencies: {}
impl std :: fmt :: Debug for WritableTarget { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , match self { Self :: WriteStdout => "stdout" , Self :: PrintStdout => "stdout" , Self :: WriteStderr => "stderr" , Self :: PrintStderr => "stderr" , Self :: Pipe (_) => "pipe" , }) } }
};
}
