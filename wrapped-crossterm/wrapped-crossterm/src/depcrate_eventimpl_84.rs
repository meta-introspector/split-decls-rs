// Generated macro for impl_84 (impl)
macro_rules! Depcrate_eventimpl_84 {
() => {
// Module: crate::event
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (feature = "bracketed-paste")] impl Command for EnableBracketedPaste { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?2004h")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Unsupported , "Bracketed paste not implemented in the legacy Windows API." ,)) } }
};
}
