// Generated macro for impl_7 (impl)
macro_rules! Depcrate_streamimpl_7 {
() => {
// Module: crate::stream
// Provides: {"impl_7"}
// Dependencies: {}
impl WinconStream for dyn std :: io :: Write { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { crate :: ansi :: write_colored (self , fg , bg , data) } }
};
}
