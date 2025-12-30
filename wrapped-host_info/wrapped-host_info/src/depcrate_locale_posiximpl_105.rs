// Generated macro for impl_105 (impl)
macro_rules! Depcrate_locale_posiximpl_105 {
() => {
// Module: crate::locale::posix
// Provides: {"impl_105"}
// Dependencies: {}
impl Delimiter { # [doc = " Find any optional sections, returning an error if the delimiters are invalid"] pub fn try_find_sections (src : & str) -> Result < Vec < (usize , Self) > , PosixParseError > { let optional_sections = src . chars () . enumerate () . flat_map (| (index , character) | match character { '_' => Some ((index , Self :: Territory)) , '.' => Some ((index , Self :: Codeset)) , '@' => Some ((index , Self :: Modifier)) , _ => None , }) . collect :: < Vec < _ > > () ; for (index , (first_offset , first_delimiter)) in optional_sections . iter () . enumerate () { if let Some ((second_offset , _second_delimiter)) = optional_sections . iter () . skip (index + 1) . find (| (_second_offset , second_delimiter) | first_delimiter == second_delimiter) { return Err (PosixParseError :: RepeatedDelimiter { first_offset : * first_offset , second_offset : * second_offset , }) ; } if let Some ((second_offset , second_delimiter)) = optional_sections . get (index + 1) { if first_delimiter > second_delimiter { return Err (PosixParseError :: UnorderedDelimiter { first_offset : * first_offset , second_offset : * second_offset , }) ; } } } Ok (optional_sections) } }
};
}
