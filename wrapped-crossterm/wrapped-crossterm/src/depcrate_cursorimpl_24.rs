// Generated macro for impl_24 (impl)
macro_rules! Depcrate_cursorimpl_24 {
() => {
// Module: crate::cursor
// Provides: {"impl_24"}
// Dependencies: {}
impl Command for MoveLeft { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}D") , self . 0) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: move_left (self . 0) } }
};
}
