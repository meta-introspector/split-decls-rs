// Generated macro for write_colored (function)
macro_rules! Depcrate_windowswrite_colored {
() => {
// Module: crate::windows
// Provides: {"write_colored"}
// Dependencies: {}
pub (crate) fn write_colored < S : AsHandle + std :: io :: Write > (stream : & mut S , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] , initial : StdioColorResult ,) -> std :: io :: Result < usize > { let (initial_fg , initial_bg) = initial ? ; let non_default = fg . is_some () || bg . is_some () ; if non_default { let fg = fg . unwrap_or (initial_fg) ; let bg = bg . unwrap_or (initial_bg) ; stream . flush () ? ; set_colors (stream , fg , bg) ? ; } let written = stream . write (data) ? ; if non_default { stream . flush () ? ; set_colors (stream , initial_fg , initial_bg) ? ; } Ok (written) }
};
}
