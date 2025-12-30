// Generated macro for impl_107 (impl)
macro_rules! Depcrate_locale_posiximpl_107 {
() => {
// Module: crate::locale::posix
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'src > PosixLocale < 'src > { # [doc = " Attempt to parse a POSIX locale."] pub fn try_from_str (src : & 'src str) -> Result < Self , PosixParseError > { if src . is_empty () { return Err (PosixParseError :: EmptyLocale) ; } if let Some (offset) = src . find ('/') { return Err (PosixParseError :: InvalidCharacter { offset }) ; } if src == "." || src == ".." { return Err (PosixParseError :: InvalidLocale) ; } let optional_sections = Delimiter :: try_find_sections (src) ? ; let language = match optional_sections . first () { Some ((offset , _delimiter)) => & src [.. * offset] , None => src , } ; if language . is_empty () { return Err (PosixParseError :: EmptySection { offset : 0 }) ; } let mut locale = Self { language , territory : None , codeset : None , modifier : None , } ; for (index , (start_offset , delimiter)) in optional_sections . iter () . enumerate () { let end_offset = optional_sections . get (index + 1) . map (| (next_offset , _next_delimiter) | * next_offset) . unwrap_or (src . len ()) ; if start_offset + 1 >= end_offset { return Err (PosixParseError :: EmptySection { offset : * start_offset , }) ; } let section_value = Some (& src [start_offset + 1 .. end_offset]) ; match delimiter { Delimiter :: Territory => locale . territory = section_value , Delimiter :: Codeset => locale . codeset = section_value , Delimiter :: Modifier => locale . modifier = section_value , } } Ok (locale) } }
};
}
