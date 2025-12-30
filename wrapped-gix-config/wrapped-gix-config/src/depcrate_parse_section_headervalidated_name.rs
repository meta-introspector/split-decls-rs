// Generated macro for validated_name (function)
macro_rules! Depcrate_parse_section_headervalidated_name {
() => {
// Module: crate::parse::section::header
// Provides: {"validated_name"}
// Dependencies: {}
fn validated_name (name : Cow < '_ , BStr >) -> Result < Cow < '_ , BStr > , Error > { name . iter () . all (| b | b . is_ascii_alphanumeric () || * b == b'-') . then_some (name) . ok_or (Error :: InvalidName) }
};
}
