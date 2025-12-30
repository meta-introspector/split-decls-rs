// Generated macro for impl_143 (impl)
macro_rules! Depcrate_styleimpl_143 {
() => {
// Module: crate::style
// Provides: {"impl_143"}
// Dependencies: {}
impl < D : Display > Command for PrintStyledContent < D > { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { let style = self . 0 . style () ; let mut reset_background = false ; let mut reset_foreground = false ; let mut reset = false ; if let Some (bg) = style . background_color { execute_fmt (f , SetBackgroundColor (bg)) . map_err (| _ | fmt :: Error) ? ; reset_background = true ; } if let Some (fg) = style . foreground_color { execute_fmt (f , SetForegroundColor (fg)) . map_err (| _ | fmt :: Error) ? ; reset_foreground = true ; } if let Some (ul) = style . underline_color { execute_fmt (f , SetUnderlineColor (ul)) . map_err (| _ | fmt :: Error) ? ; reset_foreground = true ; } if ! style . attributes . is_empty () { execute_fmt (f , SetAttributes (style . attributes)) . map_err (| _ | fmt :: Error) ? ; reset = true ; } write ! (f , "{}" , self . 0 . content ()) ? ; if reset { execute_fmt (f , ResetColor) . map_err (| _ | fmt :: Error) ? ; } else { if reset_background { execute_fmt (f , SetBackgroundColor (Color :: Reset)) . map_err (| _ | fmt :: Error) ? ; } if reset_foreground { execute_fmt (f , SetForegroundColor (Color :: Reset)) . map_err (| _ | fmt :: Error) ? ; } } Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
