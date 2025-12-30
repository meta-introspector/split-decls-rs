// Generated macro for impl_34 (impl)
macro_rules! Depcrate_cursorimpl_34 {
() => {
// Module: crate::cursor
// Provides: {"impl_34"}
// Dependencies: {}
impl Command for EnableBlinking { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?12h")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
