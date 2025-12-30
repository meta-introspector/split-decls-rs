// Generated macro for impl_86 (impl)
macro_rules! Depcrate_eventimpl_86 {
() => {
// Module: crate::event
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (feature = "bracketed-paste")] impl Command for DisableBracketedPaste { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?2004l")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
