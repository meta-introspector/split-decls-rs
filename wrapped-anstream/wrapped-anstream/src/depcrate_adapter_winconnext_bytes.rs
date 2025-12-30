// Generated macro for next_bytes (function)
macro_rules! Depcrate_adapter_winconnext_bytes {
() => {
// Module: crate::adapter::wincon
// Provides: {"next_bytes"}
// Dependencies: {}
# [inline] fn next_bytes (bytes : & mut & [u8] , parser : & mut anstyle_parse :: Parser , capture : & mut WinconCapture ,) -> Option < (anstyle :: Style , String) > { capture . reset () ; while capture . ready . is_none () { let byte = if let Some ((byte , remainder)) = (* bytes) . split_first () { * bytes = remainder ; * byte } else { break ; } ; parser . advance (capture , byte) ; } if capture . printable . is_empty () { return None ; } let style = capture . ready . unwrap_or (capture . style) ; Some ((style , std :: mem :: take (& mut capture . printable))) }
};
}
