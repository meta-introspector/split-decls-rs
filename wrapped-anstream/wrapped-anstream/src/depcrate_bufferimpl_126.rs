// Generated macro for impl_126 (impl)
macro_rules! Depcrate_bufferimpl_126 {
() => {
// Module: crate::buffer
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (all (windows , feature = "wincon"))] impl anstyle_wincon :: WinconStream for Buffer { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { self . 0 . write_colored (fg , bg , data) } }
};
}
