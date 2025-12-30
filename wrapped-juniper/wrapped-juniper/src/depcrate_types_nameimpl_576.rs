// Generated macro for impl_576 (impl)
macro_rules! Depcrate_types_nameimpl_576 {
() => {
// Module: crate::types::name
// Provides: {"impl_576"}
// Dependencies: {}
impl Name { # [doc = " Creates a new [`Name`] out of the provided `input` string, if it [`is_valid`]."] # [doc = ""] # [doc = " [`is_valid`]: Name::is_valid"] pub fn new < S > (input : S) -> Result < Self , NameParseError > where S : AsRef < str > + Into < ArcStr > , { if Self :: is_valid (input . as_ref ()) { Ok (Name (input . into ())) } else { Err (NameParseError (arcstr :: format ! ("`Name` must match /^[_a-zA-Z][_a-zA-Z0-9]*$/ but \"{}\" does not" , input . as_ref () ,))) } } # [doc = " Validates the provided `input` string to represent a valid [`Name`]."] # [must_use] pub fn is_valid (input : & str) -> bool { for (i , c) in input . chars () . enumerate () { let is_valid = c . is_ascii_alphabetic () || c == '_' || (i > 0 && c . is_ascii_digit ()) ; if ! is_valid { return false ; } } ! input . is_empty () } }
};
}
