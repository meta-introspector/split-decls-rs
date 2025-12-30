// Generated macro for impl_131 (impl)
macro_rules! Depcrate_styleimpl_131 {
() => {
// Module: crate::style
// Provides: {"impl_131"}
// Dependencies: {}
impl Command for SetBackgroundColor { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("{}m") , Colored :: BackgroundColor (self . 0)) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: windows :: set_background_color (self . 0) } }
};
}
