// Generated macro for impl_8 (impl)
macro_rules! Depcrate_streamimpl_8 {
() => {
// Module: crate::stream
// Provides: {"impl_8"}
// Dependencies: {}
impl WinconStream for dyn std :: io :: Write + Send { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { crate :: ansi :: write_colored (self , fg , bg , data) } }
};
}
