// Generated macro for normalize_xml10_eols (function)
macro_rules! Depcrate_escapenormalize_xml10_eols {
() => {
// Module: crate::escape
// Provides: {"normalize_xml10_eols"}
// Dependencies: {}
pub (crate) fn normalize_xml10_eols < 'input > (text : & 'input str) -> Cow < 'input , str > { let bytes = text . as_bytes () ; if let Some (i) = memchr (b'\r' , bytes) { let mut normalized = String :: with_capacity (text . len ()) ; normalized . push_str (& text [0 .. i]) ; let mut pos = normalize_xml10_eol_step (& mut normalized , bytes , i , '\n') ; while let Some (i) = memchr (b'\r' , & bytes [pos ..]) { let index = pos + i ; normalized . push_str (& text [pos .. index]) ; pos = normalize_xml10_eol_step (& mut normalized , bytes , index , '\n') ; } if let Some (rest) = text . get (pos ..) { normalized . push_str (rest) ; } return normalized . into () ; } Cow :: Borrowed (text) }
};
}
