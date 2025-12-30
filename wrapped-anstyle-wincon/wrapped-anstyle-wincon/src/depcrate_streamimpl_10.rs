// Generated macro for impl_10 (impl)
macro_rules! Depcrate_streamimpl_10 {
() => {
// Module: crate::stream
// Provides: {"impl_10"}
// Dependencies: {}
impl WinconStream for std :: fs :: File { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { crate :: ansi :: write_colored (self , fg , bg , data) } }
};
}
