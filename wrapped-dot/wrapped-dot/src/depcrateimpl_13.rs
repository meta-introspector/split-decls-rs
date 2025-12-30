// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > Id < 'a > { # [doc = " Creates an `Id` named `name`."] # [doc = ""] # [doc = " The caller must ensure that the input conforms to an"] # [doc = " identifier format: it must be a non-empty string made up of"] # [doc = " alphanumeric or underscore characters, not beginning with a"] # [doc = " digit (i.e. the regular expression `[a-zA-Z_][a-zA-Z_0-9]*`)."] # [doc = ""] # [doc = " (Note: this format is a strict subset of the `ID` format"] # [doc = " defined by the DOT language.  This function may change in the"] # [doc = " future to accept a broader subset, or the entirety, of DOT's"] # [doc = " `ID` format.)"] # [doc = ""] # [doc = " Passing an invalid string (containing spaces, brackets,"] # [doc = " quotes, ...) will return an empty `Err` value."] pub fn new < Name : Into < Cow < 'a , str > > > (name : Name) -> Result < Id < 'a > , & 'static str > { let name = name . into () ; { let mut chars = name . chars () ; match chars . next () { Some (c) if is_letter_or_underscore (c) => { } _ => return Err ("First character is not a letter or an underscore") , } if ! chars . all (is_constituent) { return Err ("Contains characters which are not alphanumeric/underscore characters") ; } } return Ok (Id { name }) ; fn is_letter_or_underscore (c : char) -> bool { c . is_ascii_alphabetic () || c == '_' } fn is_constituent (c : char) -> bool { is_letter_or_underscore (c) || c . is_ascii_digit () } } pub fn as_slice (& 'a self) -> & 'a str { & self . name } pub fn name (self) -> Cow < 'a , str > { self . name } }
};
}
