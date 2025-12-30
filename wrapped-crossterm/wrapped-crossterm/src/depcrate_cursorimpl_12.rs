// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cursorimpl_12 {
() => {
// Module: crate::cursor
// Provides: {"impl_12"}
// Dependencies: {}
impl Command for MoveToPreviousLine { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}F") , self . 0) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { if self . 0 != 0 { sys :: move_to_previous_line (self . 0) ? ; } Ok (()) } }
};
}
