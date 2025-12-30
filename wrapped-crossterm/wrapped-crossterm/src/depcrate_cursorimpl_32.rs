// Generated macro for impl_32 (impl)
macro_rules! Depcrate_cursorimpl_32 {
() => {
// Module: crate::cursor
// Provides: {"impl_32"}
// Dependencies: {}
impl Command for Show { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?25h")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: show_cursor (true) } }
};
}
