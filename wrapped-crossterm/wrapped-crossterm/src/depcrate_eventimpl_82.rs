// Generated macro for impl_82 (impl)
macro_rules! Depcrate_eventimpl_82 {
() => {
// Module: crate::event
// Provides: {"impl_82"}
// Dependencies: {}
impl Command for DisableFocusChange { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?1004l")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
