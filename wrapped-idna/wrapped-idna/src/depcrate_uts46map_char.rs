// Generated macro for map_char (function)
macro_rules! Depcrate_uts46map_char {
() => {
// Module: crate::uts46
// Provides: {"map_char"}
// Dependencies: {}
fn map_char (codepoint : char , flags : Flags , output : & mut String , errors : & mut Vec < Error >) { match * find_char (codepoint) { Mapping :: Valid => output . push (codepoint) , Mapping :: Ignored => { } , Mapping :: Mapped (ref slice) => output . push_str (decode_slice (slice)) , Mapping :: Deviation (ref slice) => { if flags . transitional_processing { output . push_str (decode_slice (slice)) } else { output . push (codepoint) } } Mapping :: Disallowed => { errors . push (Error :: DissallowedCharacter) ; output . push (codepoint) ; } Mapping :: DisallowedStd3Valid => { if flags . use_std3_ascii_rules { errors . push (Error :: DissallowedByStd3AsciiRules) ; } output . push (codepoint) } Mapping :: DisallowedStd3Mapped (ref slice) => { if flags . use_std3_ascii_rules { errors . push (Error :: DissallowedMappedInStd3) ; } output . push_str (decode_slice (slice)) } } }
};
}
