// Generated macro for impl_88 (impl)
macro_rules! Depcrate_eventimpl_88 {
() => {
// Module: crate::event
// Provides: {"impl_88"}
// Dependencies: {}
impl Command for PushKeyboardEnhancementFlags { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , "{}{}u" , csi ! (">") , self . 0 . bits ()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { use std :: io ; Err (io :: Error :: new (io :: ErrorKind :: Unsupported , "Keyboard progressive enhancement not implemented for the legacy Windows API." ,)) } # [cfg (windows)] fn is_ansi_code_supported (& self) -> bool { false } }
};
}
