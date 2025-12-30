// Generated macro for impl_129 (impl)
macro_rules! Depcrate_styleimpl_129 {
() => {
// Module: crate::style
// Provides: {"impl_129"}
// Dependencies: {}
impl Command for SetForegroundColor { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}m") , Colored :: ForegroundColor (self . 0)) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: windows :: set_foreground_color (self . 0) } }
};
}
