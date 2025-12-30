// Generated macro for impl_193 (impl)
macro_rules! Depcrate_process_windowsimpl_193 {
() => {
// Module: crate::process::windows
// Provides: {"impl_193"}
// Dependencies: {}
impl ProcessStream { fn new (output : PipeReader , input : PipeWriter) -> Self { Self { input , output } } # [doc = " Tries to clone the stream."] pub fn try_clone (& self) -> std :: result :: Result < Self , conpty :: error :: Error > { Ok (Self { input : self . input . try_clone () ? , output : self . output . try_clone () ? , }) } }
};
}
