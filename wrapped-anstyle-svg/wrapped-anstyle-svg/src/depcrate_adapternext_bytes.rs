// Generated macro for next_bytes (function)
macro_rules! Depcrate_adapternext_bytes {
() => {
// Module: crate::adapter
// Provides: {"next_bytes"}
// Dependencies: {}
# [inline] fn next_bytes (bytes : & mut & [u8] , parser : & mut anstyle_parse :: Parser , capture : & mut AnsiCapture ,) -> Option < Element > { capture . reset () ; while capture . ready . is_none () { let byte = if let Some ((byte , remainder)) = (* bytes) . split_first () { * bytes = remainder ; * byte } else { break ; } ; parser . advance (capture , byte) ; } if capture . printable . is_empty () { return None ; } let (style , url) = capture . ready . clone () . unwrap_or ((capture . style , None)) ; Some (Element { text : std :: mem :: take (& mut capture . printable) , style , url , }) }
};
}
