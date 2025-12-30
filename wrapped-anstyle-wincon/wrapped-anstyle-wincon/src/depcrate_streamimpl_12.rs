// Generated macro for impl_12 (impl)
macro_rules! Depcrate_streamimpl_12 {
() => {
// Module: crate::stream
// Provides: {"impl_12"}
// Dependencies: {}
impl WinconStream for std :: io :: Stdout { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { self . lock () . write_colored (fg , bg , data) } }
};
}
