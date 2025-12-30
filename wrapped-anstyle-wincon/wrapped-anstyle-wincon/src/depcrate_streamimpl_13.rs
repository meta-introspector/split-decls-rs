// Generated macro for impl_13 (impl)
macro_rules! Depcrate_streamimpl_13 {
() => {
// Module: crate::stream
// Provides: {"impl_13"}
// Dependencies: {}
impl WinconStream for std :: io :: Stderr { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { self . lock () . write_colored (fg , bg , data) } }
};
}
