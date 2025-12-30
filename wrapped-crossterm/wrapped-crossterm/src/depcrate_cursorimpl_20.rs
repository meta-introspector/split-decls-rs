// Generated macro for impl_20 (impl)
macro_rules! Depcrate_cursorimpl_20 {
() => {
// Module: crate::cursor
// Provides: {"impl_20"}
// Dependencies: {}
impl Command for MoveRight { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}C") , self . 0) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: move_right (self . 0) } }
};
}
