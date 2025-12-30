// Generated macro for impl_fmt (macro)
macro_rules! Depcrate_utilsimpl_fmt {
() => {
// Module: crate::utils
// Provides: {"impl_fmt"}
// Dependencies: {}
macro_rules ! impl_fmt { ($ name : ident) => { impl < D : fmt ::$ name > fmt ::$ name for StyledObject < D > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut reset = false ; if self . style . force . unwrap_or_else (|| match self . style . for_stderr { true => colors_enabled_stderr () , false => colors_enabled () , }) { if let Some (fg) = self . style . fg { if let Color :: TrueColor (r , g , b) = fg { write ! (f , "\x1b[38;2;{};{};{}m" , r , g , b) ?; } else if fg . is_color256 () { write ! (f , "\x1b[38;5;{}m" , fg . ansi_num ()) ?; } else if self . style . fg_bright { write ! (f , "\x1b[38;5;{}m" , fg . ansi_num () + 8) ?; } else { write ! (f , "\x1b[{}m" , fg . ansi_num () + 30) ?; } reset = true ; } if let Some (bg) = self . style . bg { if let Color :: TrueColor (r , g , b) = bg { write ! (f , "\x1b[48;2;{};{};{}m" , r , g , b) ?; } else if bg . is_color256 () { write ! (f , "\x1b[48;5;{}m" , bg . ansi_num ()) ?; } else if self . style . bg_bright { write ! (f , "\x1b[48;5;{}m" , bg . ansi_num () + 8) ?; } else { write ! (f , "\x1b[{}m" , bg . ansi_num () + 40) ?; } reset = true ; } if ! self . style . attrs . is_empty () { write ! (f , "{}" , self . style . attrs) ?; reset = true ; } } fmt ::$ name :: fmt (& self . val , f) ?; if reset { write ! (f , "\x1b[0m") ?; } Ok (()) } } } ; }
};
}
