// Generated macro for impl_9 (impl)
macro_rules! Depcrate_streamimpl_9 {
() => {
// Module: crate::stream
// Provides: {"impl_9"}
// Dependencies: {}
impl WinconStream for dyn std :: io :: Write + Send + Sync { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { crate :: ansi :: write_colored (self , fg , bg , data) } }
};
}
