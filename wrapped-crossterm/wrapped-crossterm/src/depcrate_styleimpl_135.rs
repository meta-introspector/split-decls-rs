// Generated macro for impl_135 (impl)
macro_rules! Depcrate_styleimpl_135 {
() => {
// Module: crate::style
// Provides: {"impl_135"}
// Dependencies: {}
impl Command for SetColors { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { match (self . 0 . foreground , self . 0 . background) { (Some (fg) , Some (bg)) => { write ! (f , csi ! ("{};{}m") , Colored :: ForegroundColor (fg) , Colored :: BackgroundColor (bg)) } (Some (fg) , None) => write ! (f , csi ! ("{}m") , Colored :: ForegroundColor (fg)) , (None , Some (bg)) => write ! (f , csi ! ("{}m") , Colored :: BackgroundColor (bg)) , (None , None) => Ok (()) , } } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { if let Some (color) = self . 0 . foreground { sys :: windows :: set_foreground_color (color) ? ; } if let Some (color) = self . 0 . background { sys :: windows :: set_background_color (color) ? ; } Ok (()) } }
};
}
