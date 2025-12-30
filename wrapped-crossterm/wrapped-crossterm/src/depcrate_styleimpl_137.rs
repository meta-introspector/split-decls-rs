// Generated macro for impl_137 (impl)
macro_rules! Depcrate_styleimpl_137 {
() => {
// Module: crate::style
// Provides: {"impl_137"}
// Dependencies: {}
impl Command for SetAttribute { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}m") , self . 0 . sgr ()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
