// Generated macro for impl_36 (impl)
macro_rules! Depcrate_cursorimpl_36 {
() => {
// Module: crate::cursor
// Provides: {"impl_36"}
// Dependencies: {}
impl Command for DisableBlinking { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?12l")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
