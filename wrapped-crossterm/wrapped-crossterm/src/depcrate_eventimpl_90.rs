// Generated macro for impl_90 (impl)
macro_rules! Depcrate_eventimpl_90 {
() => {
// Module: crate::event
// Provides: {"impl_90"}
// Dependencies: {}
impl Command for PopKeyboardEnhancementFlags { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("<1u")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { use std :: io ; Err (io :: Error :: new (io :: ErrorKind :: Unsupported , "Keyboard progressive enhancement not implemented for the legacy Windows API." ,)) } # [cfg (windows)] fn is_ansi_code_supported (& self) -> bool { false } }
};
}
