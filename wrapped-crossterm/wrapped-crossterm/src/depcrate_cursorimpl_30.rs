// Generated macro for impl_30 (impl)
macro_rules! Depcrate_cursorimpl_30 {
() => {
// Module: crate::cursor
// Provides: {"impl_30"}
// Dependencies: {}
impl Command for Hide { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?25l")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: show_cursor (false) } }
};
}
