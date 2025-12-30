// Generated macro for impl_11 (impl)
macro_rules! Depcrate_streamimpl_11 {
() => {
// Module: crate::stream
// Provides: {"impl_11"}
// Dependencies: {}
impl WinconStream for Vec < u8 > { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { crate :: ansi :: write_colored (self , fg , bg , data) } }
};
}
