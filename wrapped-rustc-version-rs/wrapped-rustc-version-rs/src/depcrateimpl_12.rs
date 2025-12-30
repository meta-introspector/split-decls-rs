// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl FromStr for LlvmVersion { type Err = LlvmVersionParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut parts = s . split ('.') . map (| part | -> Result < u64 , LlvmVersionParseError > { if part == "0" { Ok (0) } else if part . starts_with ('0') { Err (LlvmVersionParseError :: ComponentMustNotHaveLeadingZeros) } else if part . starts_with ('-') || part . starts_with ('+') { Err (LlvmVersionParseError :: ComponentMustNotHaveSign) } else { Ok (part . parse () ?) } }) ; let major = parts . next () . unwrap () ? ; let mut minor = 0 ; if let Some (part) = parts . next () { minor = part ? ; } else if major < 4 { return Err (LlvmVersionParseError :: MinorVersionRequiredBefore4) ; } if let Some (Err (e)) = parts . next () { return Err (e) ; } if parts . next () . is_some () { return Err (LlvmVersionParseError :: TooManyComponents) ; } Ok (Self { major , minor }) } }
};
}
