// Generated macro for impl_26 (impl)
macro_rules! Depcrate_cursorimpl_26 {
() => {
// Module: crate::cursor
// Provides: {"impl_26"}
// Dependencies: {}
impl Command for SavePosition { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str ("\x1B7") } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: save_position () } }
};
}
