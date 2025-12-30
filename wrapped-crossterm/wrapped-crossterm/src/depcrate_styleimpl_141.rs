// Generated macro for impl_141 (impl)
macro_rules! Depcrate_styleimpl_141 {
() => {
// Module: crate::style
// Provides: {"impl_141"}
// Dependencies: {}
impl Command for SetStyle { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { if let Some (bg) = self . 0 . background_color { execute_fmt (f , SetBackgroundColor (bg)) . map_err (| _ | fmt :: Error) ? ; } if let Some (fg) = self . 0 . foreground_color { execute_fmt (f , SetForegroundColor (fg)) . map_err (| _ | fmt :: Error) ? ; } if let Some (ul) = self . 0 . underline_color { execute_fmt (f , SetUnderlineColor (ul)) . map_err (| _ | fmt :: Error) ? ; } if ! self . 0 . attributes . is_empty () { execute_fmt (f , SetAttributes (self . 0 . attributes)) . map_err (| _ | fmt :: Error) ? ; } Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { panic ! ("tried to execute SetStyle command using WinAPI, use ANSI instead") ; } # [cfg (windows)] fn is_ansi_code_supported (& self) -> bool { true } }
};
}
