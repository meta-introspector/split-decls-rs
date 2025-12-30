// Generated macro for impl_28 (impl)
macro_rules! Depcrate_cursorimpl_28 {
() => {
// Module: crate::cursor
// Provides: {"impl_28"}
// Dependencies: {}
impl Command for RestorePosition { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str ("\x1B8") } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: restore_position () } }
};
}
