// Generated macro for impl_10 (impl)
macro_rules! Depcrate_cursorimpl_10 {
() => {
// Module: crate::cursor
// Provides: {"impl_10"}
// Dependencies: {}
impl Command for MoveToNextLine { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}E") , self . 0) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { if self . 0 != 0 { sys :: move_to_next_line (self . 0) ? ; } Ok (()) } }
};
}
