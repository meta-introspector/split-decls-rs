// Generated macro for impl_16 (impl)
macro_rules! Depcrate_cursorimpl_16 {
() => {
// Module: crate::cursor
// Provides: {"impl_16"}
// Dependencies: {}
impl Command for MoveToRow { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}d") , self . 0 + 1) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: move_to_row (self . 0) } }
};
}
