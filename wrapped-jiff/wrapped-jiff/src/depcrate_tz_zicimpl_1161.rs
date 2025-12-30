// Generated macro for impl_1161 (impl)
macro_rules! Depcrate_tz_zicimpl_1161 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1161"}
// Dependencies: {}
impl < 'a > FieldParser < 'a > { # [doc = " Create a new parser from a UTF-8 encoded sequence of bytes."] fn new (src : & 'a str) -> FieldParser { FieldParser { lines : src . lines () , line_number : 0 , fields : vec ! [] , continuation_zone_for : None , } } # [doc = " Create a new parser from a sequence of bytes."] # [doc = ""] # [doc = " This returns an error if the given bytes are not valid UTF-8."] fn from_bytes (src : & 'a [u8]) -> Result < FieldParser , Error > { let src = core :: str :: from_utf8 (src) . map_err (| e | err ! ("invalid UTF-8: {e}")) ? ; Ok (FieldParser :: new (src)) } # [doc = " Advances the parser's line iterator and splits it into `self.fields`."] # [doc = ""] # [doc = " If there are no more lines, then this returns `Ok(false)`. Otherwise,"] # [doc = " if the next line exists and was successfully parsed into a sequence of"] # [doc = " fields, then `Ok(true)` is returned."] # [doc = ""] # [doc = " This guarantees that when `true` is returned, `self.fields` is"] # [doc = " non-empty."] fn read_next_fields (& mut self) -> Result < bool , Error > { self . fields . clear () ; loop { let Some (mut line) = self . lines . next () else { return Ok (false) } ; self . line_number = self . line_number . checked_add (1) . ok_or_else (| | err ! ("line count overflowed")) ? ; parse_fields (& line , & mut self . fields) . with_context (| | err ! ("line {}" , self . line_number)) ? ; if self . fields . is_empty () { continue ; } return Ok (true) ; } } }
};
}
