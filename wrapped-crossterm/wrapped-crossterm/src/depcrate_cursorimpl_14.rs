// Generated macro for impl_14 (impl)
macro_rules! Depcrate_cursorimpl_14 {
() => {
// Module: crate::cursor
// Provides: {"impl_14"}
// Dependencies: {}
impl Command for MoveToColumn { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}G") , self . 0 + 1) ? ; Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: move_to_column (self . 0) } }
};
}
