// Generated macro for impl_6 (impl)
macro_rules! Depcrate_streamimpl_6 {
() => {
// Module: crate::stream
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : WinconStream + ? Sized > WinconStream for Box < T > { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { (* * self) . write_colored (fg , bg , data) } }
};
}
