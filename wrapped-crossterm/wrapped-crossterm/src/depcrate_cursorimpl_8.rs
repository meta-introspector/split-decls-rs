// Generated macro for impl_8 (impl)
macro_rules! Depcrate_cursorimpl_8 {
() => {
// Module: crate::cursor
// Provides: {"impl_8"}
// Dependencies: {}
impl Command for MoveTo { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{};{}H") , self . 1 + 1 , self . 0 + 1) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: move_to (self . 0 , self . 1) } }
};
}
