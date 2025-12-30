// Generated macro for impl_22 (impl)
macro_rules! Depcrate_cursorimpl_22 {
() => {
// Module: crate::cursor
// Provides: {"impl_22"}
// Dependencies: {}
impl Command for MoveDown { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}B") , self . 0) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: move_down (self . 0) } }
};
}
