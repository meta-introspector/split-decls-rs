// Generated macro for impl_5 (impl)
macro_rules! Depcrate_streamimpl_5 {
() => {
// Module: crate::stream
// Provides: {"impl_5"}
// Dependencies: {}
impl < T : WinconStream + ? Sized > WinconStream for & mut T { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { (* * self) . write_colored (fg , bg , data) } }
};
}
