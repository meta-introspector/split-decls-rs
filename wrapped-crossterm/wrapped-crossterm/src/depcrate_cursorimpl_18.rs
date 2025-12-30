// Generated macro for impl_18 (impl)
macro_rules! Depcrate_cursorimpl_18 {
() => {
// Module: crate::cursor
// Provides: {"impl_18"}
// Dependencies: {}
impl Command for MoveUp { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}A") , self . 0) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: move_up (self . 0) } }
};
}
