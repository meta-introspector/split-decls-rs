// Generated macro for _escape (function)
macro_rules! Depcrate_escape_escape {
() => {
// Module: crate::escape
// Provides: {"_escape"}
// Dependencies: {}
# [doc = " Escapes an `&str` and replaces a subset of xml special characters (`<`, `>`,"] # [doc = " `&`, `'`, `\"`) with their corresponding xml escaped value."] fn _escape < 'a , F : Fn (u8) -> bool > (raw : impl Into < Cow < 'a , str > > , escape_chars : F) -> Cow < 'a , str > { let raw = raw . into () ; let bytes = raw . as_bytes () ; let mut escaped = None ; let mut iter = bytes . iter () ; let mut pos = 0 ; while let Some (i) = iter . position (| & b | escape_chars (b)) { if escaped . is_none () { escaped = Some (String :: with_capacity (raw . len ())) ; } let escaped = escaped . as_mut () . expect ("initialized") ; let new_pos = pos + i ; escape_char (escaped , & raw , pos , new_pos) . unwrap () ; pos = new_pos + 1 ; } if let Some (mut escaped) = escaped { if let Some (raw) = raw . get (pos ..) { escaped . write_str (raw) . unwrap () ; } Cow :: Owned (escaped) } else { raw } }
};
}
